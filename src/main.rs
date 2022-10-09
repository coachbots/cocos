#[macro_use]
extern crate lazy_static;

use config::APP_CONFIG;
use controllers::master::MasterController;
use io::rpi::gpio::RpiGpioDriver;
use io::rpi::pwm::RpiPwmDriver;
use io::rpi::uart::RpiUartDriver;

mod io;
mod drivers;
mod config;
mod controllers;
mod models;

lazy_static! {
    static ref MASTER_CONTROLLER: MasterController <RpiGpioDriver,
                                                    RpiPwmDriver,
                                                    RpiUartDriver> =
    MasterController::new(
        &APP_CONFIG,
        RpiGpioDriver::new(),
        RpiPwmDriver::new(),
        RpiUartDriver::new()
    );
}

fn main() {
    (*MASTER_CONTROLLER).run()
}
