#[macro_use]
extern crate lazy_static;

use config::APP_CONFIG;
use controllers::master::MasterController;
use io::rpi::gpio::RpiGpioDriver;
use io::rpi::pwm::RpiPwmDriver;
use io::rpi::uart::RpiUartDriver;
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
    //static ref MASTER_CONTROLLER: MasterController <RpiGpioDriver,
    //                                                RpiPwmDriver,
    //                                                RpiUartDriver> =
    //MasterController::new(
    //    &APP_CONFIG,
    //    RpiGpioDriver::new(),
    //    RpiPwmDriver::new(),
    //    RpiUartDriver::new()
    //);
    static ref MASTER_CONTROLLER: MasterController<PrintGpioDriver,
                                                   PrintPwmDriver,
                                                   PrintUartDriver> =
    MasterController::new(
        &APP_CONFIG,
        PrintGpioDriver::new(),
        PrintPwmDriver::new(),
        PrintUartDriver::new()
    );
}

fn main() {
    env_logger::init();
    (*MASTER_CONTROLLER).run()
}
