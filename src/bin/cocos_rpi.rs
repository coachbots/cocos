use cocos::config::APP_CONFIG;
use cocos::controllers::master::MasterController;
use cocos::io::rpi::gpio::RpiGpioDriver;
use cocos::io::rpi::pwm::RpiPwmDriver;
use cocos::io::rpi::uart::RpiUartDriver;

fn main() {
    env_logger::init();

    let master_controller = MasterController::new(
        &APP_CONFIG,
        RpiGpioDriver::new(),
        RpiPwmDriver::new(),
        RpiUartDriver::new(APP_CONFIG.nucifera.to_uart_descriptor())
    );

    master_controller.run();
}
