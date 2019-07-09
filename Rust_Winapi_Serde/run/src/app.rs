use crate::file_read::read_file_all;

use std::io::Result;
use std::io::prelude::*;
use std::fmt::Debug;
use std::process::Command;

use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum BuildConfiguration {
    Debug,
    Release,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    name : String,
    debug_path : String,
    release_path : String,
    opt_arg : String
}

impl App {
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(windows)]
    pub fn executable_name(&self) -> String {
        format!("{}.exe", self.name)
    }


    #[cfg(not(windows))]
    pub fn executable_name(&self) -> String {
        self.name.clone()
    }


    pub fn run(&self, config : &BuildConfiguration) -> std::io::Result<()> {
        let directory = match config {
            BuildConfiguration::Debug => &self.debug_path,
            BuildConfiguration::Release => &self.release_path
        };
        let file_name = format!("{}.exe", self.name());

        let mut start_app = Command::new("cmd.exe");
        start_app.arg("/c").arg("start").arg("/d").arg(directory).arg(file_name);
        if !self.opt_arg.is_empty() {
            start_app.arg(&self.opt_arg);
        }

        start_app.spawn()?;

        Ok(())
    }


    pub fn kill(&self) -> std::io::Result<()> {
        let file_name = format!("{}.exe", self.name());

        let mut kill_app = Command::new("cmd.exe");
        kill_app.arg("/c").arg("taskkill").arg("/im").arg(file_name);

        kill_app.spawn()?;

        Ok(())
    }
}


#[derive(Serialize, Deserialize)]
pub struct Apps {
    apps : Vec<App>
}

impl Apps {
    pub fn new_from_file(file_name : &str) -> Apps {
        let file_data = match read_file_all(file_name) {
            Err(e) => panic!(e),
            Ok(s) => s
        };

        match serde_json::from_str(file_data.as_str()) {
            Err(e) => panic!(e),
            Ok(val) => val
        }
    }

    pub fn app_list(&self) -> &Vec<App> {
        &self.apps
    }
}