#[macro_use]
extern crate lazy_static;

use std::fs::File;

use config::APP_CONFIG;
use controllers::master::MasterController;
use io::sim_print::{
    gpio::PrintGpioDriver,
    uart::PrintUartDriver,
    pwm::PrintPwmDriver
};
use env_logger;

pub mod io;
pub mod drivers;
pub mod config;
pub mod controllers;
pub mod models;

lazy_static! {
    static ref MASTER_CONTROLLER: MasterController<PrintGpioDriver,
                                                   PrintPwmDriver,
                                                   PrintUartDriver> =
    MasterController::new(
        &APP_CONFIG,
        PrintGpioDriver::new(File::create("sim_gpio.out").unwrap()),
        PrintPwmDriver::new(File::create("sim_pwm.out").unwrap()),
        PrintUartDriver::new()
    );
}

fn main() {
    env_logger::init();
    (*MASTER_CONTROLLER).run()
}
