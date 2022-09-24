use std::cell::RefCell;
use std::rc::Rc;

use config::APP_CONFIG;
use controllers::master::MasterController;
use io::interface::IOProvider;
use io::rpi::gpio::RpiGpioDriver;
use io::rpi::pwm::RpiPwmDriver;
use io::rpi::uart::RpiUartDriver;
use crate::models::app_state::AppState;

mod io;
mod drivers;
mod config;
mod controllers;
mod models;

fn main() {
    let io_provider = IOProvider {
        gpio_driver: Rc::new(RefCell::new(RpiGpioDriver::new())),
        pwm_driver: Rc::new(RefCell::new(RpiPwmDriver::new())),
        uart_driver: Rc::new(RefCell::new(RpiUartDriver::new()))
    };
    let app_state: AppState = AppState::zero();

    let mut controller = MasterController::new(&app_state, &APP_CONFIG,
                                               &io_provider);
}
