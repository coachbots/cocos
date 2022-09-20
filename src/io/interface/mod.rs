use self::gpio::DrivesGpio;
use self::uart::DrivesUart;
use self::pwm::DrivesPwm;

pub enum IOError {
    Unknown,
    Reinitialization,
    Uninitialized
}

pub trait IODriver {
    /// Called upon driver initialization.
    fn init(&mut self) -> Result<(), IOError>;
}

pub trait ProvidesIO {
    fn get_gpio(&self) -> Box<dyn DrivesGpio>;
    fn get_uart(&self) -> Box<dyn DrivesUart>;
    fn get_pwm(&self) -> Box<dyn DrivesPwm>;
}

pub mod gpio;
pub mod pwm;
pub mod uart;
