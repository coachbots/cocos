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
        gpio_driver: Box::new(RpiGpioDriver::new()),
        pwm_driver: Box::new(RpiPwmDriver::new()),
        uart_driver: Box::new(RpiUartDriver::new())
    };

    let app_state: AppState = AppState::zero();
    let controller = MasterController::new(&app_state, &APP_CONFIG,
                                           &io_provider);
}
