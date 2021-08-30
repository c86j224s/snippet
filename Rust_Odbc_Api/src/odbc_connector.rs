use std::sync::Arc;

use odbc_api::{Connection, Environment, sys::{AttrConnectionPooling, AttrCpMatch}};

pub struct OdbcConnector {
    env: Environment,
    connection_string: String
}

impl OdbcConnector {
    pub fn new(connection_string: &str) -> Result<Arc<OdbcConnector>, odbc_api::Error> {
        unsafe {
            Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)?;

            let mut env = Environment::new()?;

            env.set_connection_pooling_matching(AttrCpMatch::Strict)?;

            Ok(Arc::new(OdbcConnector {
                env: env,
                connection_string: connection_string.to_owned()
            }))
        }
    }

    pub fn get_conn(&self) -> Result<Connection, odbc_api::Error> {
        Ok(self.env.connect_with_connection_string(&self.connection_string)?)
    }
}
