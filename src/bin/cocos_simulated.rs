#[macro_use]
extern crate lazy_static;

use std::{fs::File, io::{stdin, Read}, process, time::Instant};

use cocos::config::APP_CONFIG;
use cocos::controllers::master::MasterController;
use env_logger;
use cocos::io::sim_print::{gpio::PrintGpioDriver, pwm::PrintPwmDriver, uart::PrintUartDriver};
use clap::Parser;

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
    user_script: Option<String>,

    /// Defines the GPIO IO communication file. Used exclusively with the SIL.
    #[arg(short, long)]
    gpio_file: String
}

lazy_static! {
    static ref BEGIN_TIME: Instant = Instant::now();
}

fn main() {
    let args = CliArgs::parse();

    env_logger::init();

    // TODO: Yikes, remove this... Killing the point of rust...
    let gpio_file1 = File::create(String::clone(&args.gpio_file)).unwrap();
    let gpio_file2 = File::create(args.gpio_file).unwrap();

    let master_controller = MasterController::new(
        &APP_CONFIG,
        PrintGpioDriver::new(gpio_file1, *BEGIN_TIME),
        PrintPwmDriver::new(gpio_file2, *BEGIN_TIME),
        PrintUartDriver::new()
    );

    match args.user_script {
        None => { master_controller.run() },
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
