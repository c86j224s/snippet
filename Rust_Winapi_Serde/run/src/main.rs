extern crate argparse;

use run::app::{Apps, BuildConfiguration};
use run::process_util::find_process_id_by_name;


struct Opts {
    command : String,
    app : String,
    release : bool
}

impl Opts {
    fn new() -> Opts {
        Opts {
            command : String::new(),
            app : String::new(),
            release : true
        }
    }
}


fn parse_args() -> Opts {
    let mut opts = Opts::new();

    {
        use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse};
        let mut ap = ArgumentParser::new();

        ap.refer(&mut opts.command).add_argument("command", Store, "Command to execute. 'start' or 'stop'").required();
        ap.refer(&mut opts.app).add_option(&["-P", "--app"], Store, "Specific app name.");
        ap.refer(&mut opts.release)
            .add_option(&["-R", "--release"], StoreTrue, "Do with release build.")
            .add_option(&["-D", "--debug"], StoreFalse, "Do with debug build.");

        ap.parse_or_exit();
    }

    opts
}



fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("arguments required.");
        return;
    }

    let apps = Apps::new_from_file("apps.json");

    if args[1].to_ascii_lowercase() == "start" {
        if args.len() < 3 {
            println!("argument required more.");
            return;
        }

        let build_config = match args[2].to_ascii_lowercase().as_str() {
            "debug" => BuildConfiguration::Debug,
            "release" => BuildConfiguration::Release,
            _ => {
                println!("build configuration should be 'debug' or 'release'");
                return;
            }
        };

        for app in apps.app_list().iter() {
            let found = match find_process_id_by_name(app.executable_name().as_str()) {
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
    }

    if args[1].to_ascii_lowercase() == "stop" {
        for app in apps.app_list().iter() {
            let executable_name = app.executable_name();
            let found = match find_process_id_by_name(executable_name.as_str()) {
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
