mod odbc_api_ext;
mod odbc_connector;
mod config;
mod protocol;

use config::Config;
use odbc_api::{DataType, InputParameter, IntoParameter, RowSetBuffer, handles::{CData, CDataMut, HasDataType, Statement}, parameter::{VarChar, VarCharBox}, sys::{ParamType, SqlDataType}};

use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(unused)] use odbc_api_ext::{StatementFunctions, ConnectionExt, StatementExt};
use odbc_connector::OdbcConnector;

use crate::protocol::Stmt;


#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Odbc(odbc_api::Error),
    StdParseInt(std::num::ParseIntError),
    Logic(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error [{}]", e),
            Self::Odbc(e) => write!(f, "odbc error [{}]", e),
            Self::StdParseInt(e) => write!(f, "parse int error [{}]", e),
            Self::Logic(description) => write!(f, "logic error [{}]", description),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<odbc_api::Error> for Error {
    fn from(e: odbc_api::Error) -> Self {
        Self::Odbc(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::StdParseInt(e)
    }
}

impl Error {
    fn logic(description: &str) -> Self {
        Self::Logic(description.to_owned())
    }
}




async fn handle_request(odbc: Arc<OdbcConnector>, request: protocol::Request) -> Result<(), Error> {

    let conn = odbc.get_conn()?;

    println!("connected");

    //let query = "SELECT TOP 1 1 AS first_val WHERE 'hello' = ?";
    //let query = "SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?";
    //let query = "SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = 'b0db954a-464f-48b6-b623-1ef5cd5e185a'";

    //let mut stmt = conn.get_prepared_statement(query)?;

    let mut stmt = match request.stmt {
        Stmt::Query(q) => conn.get_prepared_statement(&q)?,
        Stmt::Procedure(p) => conn.get_prepared_statement(&format!("{{ ? = call {} }}", p))?
    };

    let params = stmt.params()?;

    if params.len() != request.params.len() {
        return Err(Error::logic("no matched params count"))
    }

    for (idx, par) in params.into_iter().enumerate() {
        let databox: Box<dyn InputParameter> = match request.params[idx] {
            protocol::Param::S(ref s) => Box::new(VarChar::from_string(s.to_owned())),
            protocol::Param::NS(ref ns) => {
                //should support nvarchar
                Box::new(VarChar::from_string(ns.to_owned()))
            },
            protocol::Param::I(ref i) => { 
                let n: i32 = i.to_owned() as i32;
                Box::new(n.into_parameter())
            },
        };

        unsafe { stmt.bind_input_parameter(idx as u16 + 1, &databox) };
    }

    //let mut id = VarChar::from_string("3BDDC9BD-FAAE-42C0-BD28-5DB3EAA8D6E4".to_owned()); // incorrect. uniqueidentifier != varchar
    //unsafe { stmt.bind_parameter(1, ParamType::Input, &mut id)? }; // how to know param type ? 

    stmt.execute_now()?;

    loop {
        let result_cols = stmt.result_cols()?;
        if result_cols.len() == 0 {
            return Err(Error::logic("result_cols len == 0"))
        }

        while unsafe { stmt.fetch()? } {
            println!("fetch one");
            for (idx, col_desc) in result_cols.iter().enumerate() {
                let dt = match col_desc.data_type {
                    odbc_api::DataType::WVarchar { length } => "nvarchar",
                    odbc_api::DataType::Varchar { length } => "varchar", 
                    odbc_api::DataType::Other { data_type, .. } if data_type == odbc_api::sys::SqlDataType::EXT_GUID => {
                        "varchar"
                    },
                    odbc_api::DataType::Bit |
                    odbc_api::DataType::TinyInt |
                    odbc_api::DataType::SmallInt | 
                    odbc_api::DataType::Integer |
                    odbc_api::DataType::BigInt => "integer",
                    odbc_api::DataType::Other { data_type, .. } if data_type == odbc_api::sys::SqlDataType(-155) => {
                        "varchar?"
                    },
                    _ => panic!("fdsa")
                };

                println!("dt : {}", dt);

                if dt == "varchar" {
                    //stmt.bind_col(column_number, target)
                }
                //if col_desc.data_type == odbc_api::DataType::
                //col_dec.data_type

                //stmt.get_data(idx + 1, )


            }
        }

        if !stmt.more_results()? {
            break
        }
    }

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Error> {
    let conf = Config::from_file("config.json")?;

    println!("{:?}", conf);

    let odbc = OdbcConnector::new(&conf.connection_string)?;

    let req1 = r###"{
        "stmt": {
            "type": "query",
            "val_ue_not_used": "SELECT TOP 1 Id, CountryCode, Registered FROM UserProfileDb.dbo.UserCountries WHERE Id = ?",
            "value_ok": "SELECT TOP 1 * FROM UserProfileDb.dbo.UserProfiles WHERE Id = ? AND GenderCode = ?",
            "value_ok": "SELECT TOP 1 * FROM UserProfileDb.dbo.UserProfiles WHERE 'hello' = ?",
            "value": "SELECT TOP 1 * FROM UserProfileDb.dbo.UserProfiles WHERE Id = ? AND GenderCode = ? AND RealName = ?"
        },
        "params": [
            { "S": "3BDDC9BD-FAAE-42C0-BD28-5DB3EAA8D6E4" },
            { "I": 1 },
            { "NS": "홍길동" }
        ]
    }"###;
 
    let req = protocol::parse_json_request(req1)?;

    handle_request(Arc::clone(&odbc), req).await?;


    println!("Hello, world!");

    Ok(())
}

