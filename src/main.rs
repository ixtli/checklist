extern crate ansi_term;
#[macro_use]
extern crate clap;

use std::io::{self};

use clap::App;
use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;

fn run() -> io::Result<()> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("worker")
                .about("Run as a worker.")
                .arg(
                    Arg::with_name("address")
                        .short("a")
                        .long("address")
                        .help("Address of the arbiter to join.")
                        .takes_value(true),
                ),
        );

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("worker") {
        if matches.is_present("address") {
            let config_address: &str = matches.value_of("address").unwrap_or("config.json");
            println!("Would connect to arbiter {}", config_address);
        }
    }

    Ok(())
}

fn main() {
    if cfg!(target_os = "windows") {
        eprintln!("This application does not support windows :(");
        std::process::exit(1);
    }

    let result = run();
    match result {
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }

        Ok(()) => {}
    }
}
