use std::fs::File;
use std::io;
use std::io::prelude::*;

use serde_json;
use serde::{Serialize, Deserialize};

pub fn read_file_all(file_name : &str) -> std::io::Result<String> {
    let mut file_data = String::new();

    let file = File::open(file_name)?;
    let mut buf_reader = std::io::BufReader::new(file);
    buf_reader.read_to_string(&mut file_data)?;
    Ok(file_data)
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    login_name : String,
    password : String
}

impl Account {
    pub fn login_name(&self) -> &str {
        &self.login_name
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sample {
    server_ip : String,
    server_port : u32,
    accounts : Vec<Account>
}

impl Sample {
    pub fn new_from_file(file_name : &str) -> Self {
        let file_content = match read_file_all(file_name) {
            Err(e) => panic!(e),
            Ok(s) => s
        };
        
        match serde_json::from_str(file_content.as_str()) {
            Err(e) => panic!(e),
            Ok(val) => val
        }
    }

    pub fn server_ip(&self) -> &str {
        &self.server_ip
    }

    pub fn server_port(&self) -> u32 {
        self.server_port
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
}


