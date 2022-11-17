#[macro_use]
extern crate lazy_static;

use std::{fs::File, time::Instant, io::{stdin, Read}, process};

use config::APP_CONFIG;
use controllers::master::MasterController;
use env_logger;
use io::sim_print::{gpio::PrintGpioDriver, pwm::PrintPwmDriver, uart::PrintUartDriver};
use clap::Parser;

pub mod config;
pub mod controllers;
pub mod drivers;
pub mod io;
pub mod models;

#[derive(Parser, Debug)]
struct CliArgs {
    /// The user script to run on initialization. Default to nothing if no user
    /// script is given. You may specify stdin via `--`.
    ///
    /// Example:
    ///
    /// cocos -u=--                # Will read via stdin until EOF
    ///
    /// cocos -u=my_user_script.py # Loads my_user_script.py as the user script.
    #[arg(short, long)]
    user_script: Option<String>
}

lazy_static! {
    static ref BEGIN_TIME: Instant = Instant::now();
    static ref MASTER_CONTROLLER: MasterController<PrintGpioDriver, PrintPwmDriver, PrintUartDriver> =
        MasterController::new(
            &APP_CONFIG,
            PrintGpioDriver::new(File::create("sim_gpio.out").unwrap(), *BEGIN_TIME),
            PrintPwmDriver::new(File::create("sim_gpio.out").unwrap(), *BEGIN_TIME),
            PrintUartDriver::new()
        );
}

fn main() {
    let args = CliArgs::parse();
    env_logger::init();
    match args.user_script {
        None => { (*MASTER_CONTROLLER).run() },
        Some(script) => {
            let mut user_script = String::new();
            if script == String::from("--") {
                stdin().read_to_string(&mut user_script).unwrap();
            } else {
                match File::open(script) {
                    Ok(mut file) => {
                        if let Err(err) = file.read_to_string(&mut user_script) {
                            log::error!("Could not read user_script. {:}", err);
                            process::exit(1);
                        }
                    }
                    Err(err) => {
                        log::error!("Could not open user_script. {:}", err);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
