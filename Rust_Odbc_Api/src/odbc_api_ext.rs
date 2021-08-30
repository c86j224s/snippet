use odbc_api::{ColumnDescription, Connection, U16String, handles::{ParameterDescription, Statement, StatementImpl}, sys::{HStmt, SmallInt, SqlReturn}};

#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(
    all(not(windows), not(feature = "static"), not(feature = "iodbc")),
    link(name = "odbc")
)]
#[cfg_attr(
    all(not(windows), feature = "static", not(feature = "iodbc")),
    link(name = "odbc", kind = "static")
)]
#[cfg_attr(
    all(not(windows), not(feature = "static"), feature = "iodbc"),
    link(name = "iodbc")
)]
#[cfg_attr(
    all(not(windows), feature = "static", feature = "iodbc"),
    link(name = "iodbc", kind = "static")
)]
extern "system" {
    pub fn SQLNumParams(statement_handles: HStmt, parameter_count_ptr: *mut SmallInt) -> SqlReturn;
}

pub trait SQLParamDescriber {
    fn num_params(&self) -> Result<i16, odbc_api::Error>;
}

impl<T> SQLParamDescriber for T where T: Statement {
    fn num_params(&self) -> Result<i16, odbc_api::Error> {
        let mut num_params = 0i16;
        let sql_ret = unsafe { SQLNumParams(self.as_sys(), &mut num_params) };
        match sql_ret {
            SqlReturn::SUCCESS => (),
            SqlReturn::SUCCESS_WITH_INFO => (),
            SqlReturn::ERROR => {
                /*
                log_diagnosis..
                */
                return Err(odbc_api::Error::NoDiagnostics)
            },
            r => panic!("Unexpected odbc function result: {:?}", r),
        }
        Ok(num_params)
    }
}

pub trait ConnectionExt {
    fn get_prepared_statement(&self, query: &str) -> Result<StatementImpl, odbc_api::Error>;
}

impl ConnectionExt for Connection<'_> {
    fn get_prepared_statement(&self, query: &str) -> Result<StatementImpl, odbc_api::Error> {
        let mut stmt = self.preallocate()?.into_statement();

        stmt.prepare(&U16String::from_str(query))?;

        Ok(stmt)
    }
}

pub trait StatementExt {
    fn params(&self) -> Result<Vec<ParameterDescription>, odbc_api::Error>;
    fn result_cols(&self) -> Result<Vec<ColumnDescription>, odbc_api::Error>;
    fn execute_now(&mut self) -> Result<bool, odbc_api::Error>;
}

impl StatementExt for StatementImpl<'_> {
    fn params(&self) -> Result<Vec<ParameterDescription>, odbc_api::Error> {
        let num_params = self.num_params()?;
        println!("num_params = {}", num_params);

        let mut result = vec![];
        for i in 0..num_params {
            let i = i as u16 + 1;   // starts with 1
            let param_desc = self.describe_param(i)?;

            println!("i = {}, nullable = {:?}, type = {:?}", i, param_desc.nullable, param_desc.data_type);

            result.push(param_desc);
        }

        Ok(result)
    }

    fn result_cols(&self) -> Result<Vec<ColumnDescription>, odbc_api::Error> {
        let num_cols = self.num_result_cols()?;
        println!("num_cols = {}", num_cols);

        let mut result = vec![];
        for i in 0..num_cols {
            let i = i as u16 + 1;   // starts with 1
            let mut col_desc = Default::default();
            self.describe_col(i, &mut col_desc)?;

            println!("i = {}, name = {:?}, nullability = {:?}, data_type = {:?}",
                i, U16String::from_vec(col_desc.name.clone()).to_string_lossy(), col_desc.nullability, col_desc.data_type);

            result.push(col_desc);
        }

        Ok(result)
    }

    fn execute_now(&mut self) -> Result<bool, odbc_api::Error> {
        let result = unsafe { self.execute()? };
        Ok(result)
    }
}
