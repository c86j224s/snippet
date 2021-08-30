mod odbc_api_ext;
mod odbc_connector;
mod config;
mod protocol;

use config::Config;

use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(unused)] use odbc_api_ext::{SQLParamDescriber, ConnectionExt, StatementExt};
use odbc_connector::OdbcConnector;

use crate::protocol::Stmt;


#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Odbc(odbc_api::Error),
    StdParseInt(std::num::ParseIntError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error [{}]", e),
            Self::Odbc(e) => write!(f, "odbc error [{}]", e),
            Self::StdParseInt(e) => write!(f, "parse int error [{}]", e),
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

    //let mut id = VarChar::from_string("b0db954a-464f-48b6-b623-1ef5cd5e185a".to_owned()); // incorrect. uniqueidentifier != varchar
    //unsafe { statement.bind_parameter(1, ParamType::Input, &mut id) }; // how to know param type ? 

    let result_cols = stmt.result_cols()?;

    let result = stmt.execute_now()?;

    println!("execute result : {}", result);

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
            "value": "SELECT TOP 1 * FROM UserProfileDb.dbo.UserProfiles WHERE Id = ?"
        },
        "params": [
            { "S": "random guid" }
        ]
    }"###;
 
    let req = protocol::parse_json_request(req1)?;

    handle_request(odbc, req).await?;


    println!("Hello, world!");

    Ok(())
}

