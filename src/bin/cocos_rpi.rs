use cocos::config::APP_CONFIG;
use cocos::controllers::master::MasterController;
use cocos::io::rpi::gpio::RpiGpioDriver;
use cocos::io::rpi::pwm::RpiPwmDriver;
use cocos::io::rpi::uart::RpiUartDriver;
use clap::Parser;
use std::{io::{stdin, Read}, fs::File};

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
}

fn main() {
    env_logger::init();
    let args = CliArgs::parse();

    let master_controller = MasterController::new(
        &APP_CONFIG,
        RpiGpioDriver::new(),
        RpiPwmDriver::new(),
        RpiUartDriver::new(APP_CONFIG.nucifera.to_uart_descriptor())
    );

    match args.user_script {
        None => {
            master_controller.run();
        }
        Some(script) => {
            let mut user_script = String::new();
            if script == String::from("--") {
                stdin().read_to_string(&mut user_script).unwrap();
            } else {
                match File::open(script) {
                    Ok(mut file) => {
                        if let Err(err) = file.read_to_string(&mut user_script) {
                            log::error!("Could not read user_script. {:}", err);
                        }
                    }
                    Err(err) => {
                        log::error!("Could not open user_script. {:}", err);
                    }
                }
            }
        }
    }

}
