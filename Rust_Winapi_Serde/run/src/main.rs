extern crate argparse;

use std::str::FromStr;

use run::app::{Apps, BuildConfiguration};
use run::process_util::{ProcAccessor, sys::Proc};


enum Command {
    start,
    stop 
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s : &str) -> Result<Command, ()> {
        match s.to_ascii_lowercase().as_str() {
            "start" => Ok(Command::start),
            "stop" => Ok(Command::stop),
            _ => Err(())
        }
    }
}

struct Opts {
    command : Command,
    app : String,
    release : bool
}

impl Opts {
    fn new_from_args() -> Opts {
        let mut opts = Opts {
            command : Command::start,
            app : String::new(),
            release : true
        };

        {
            use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse};
            let mut ap = ArgumentParser::new();

            ap.refer(&mut opts.command).required().add_argument("command", Store, "Command to execute. 'start' or 'stop'");
            ap.refer(&mut opts.app).add_option(&["-P", "--app"], Store, "Specific app name.");
            ap.refer(&mut opts.release)
                .add_option(&["-R", "--release"], StoreTrue, "Do with release build.")
                .add_option(&["-D", "--debug"], StoreFalse, "Do with debug build.");

            ap.stop_on_first_argument(true);
            ap.parse_args_or_exit();
        }

        opts
    }
}


fn main() {
    let opts = Opts::new_from_args();
    let apps = Apps::new_from_file("apps.json");

    let build_config = match opts.release {
        true => BuildConfiguration::Release,
        false => BuildConfiguration::Debug
    };

    match opts.command {
        Command::start => {
            for app in apps.app_list().iter() {
                if !opts.app.is_empty() {
                    if opts.app.to_ascii_lowercase() != app.name().to_ascii_lowercase() {
                        continue;
                    }
                }

                let found = match Proc::find_process_by_name(app.executable_name().as_str()) {
                    None => false,
                    Some(_) => true
                };

                if found {
                    println!("{} is already running.", app.name());
                    continue;
                }

                match app.run(&build_config) {
                    Err(e) => println!("{} start failed. {:?}", app.name(), e),
                    Ok(_) => println!("{} started.", app.name())
                };
            }            
        },
        Command::stop => {
            for app in apps.app_list().iter() {
                if !opts.app.is_empty() {
                    if opts.app.to_ascii_lowercase() != app.name().to_ascii_lowercase() {
                        continue;
                    }
                }

                let found = match Proc::find_process_by_name(app.executable_name().as_str()) {
                    None => false,
                    Some(_) => true
                };

                if !found {
                    println!("{} is already stopped.", app.name());
                    continue;
                }

                match app.kill() {
                    Err(e) => println!("{} stop failed. {:?}", app.name(), e),
                    Ok(_) => println!("{} stopped.", app.name())
                };

            }

        }
    }
}
