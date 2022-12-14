use self::gpio::DrivesGpio;
use self::pwm::DrivesPwm;
use self::uart::DrivesUart;

pub enum IOError {
    Unknown,
    Reinitialization,
    Uninitialized,
}

pub trait IODriver {
    /// Called upon driver initialization.
    fn init(&mut self) -> Result<(), IOError>;
}

pub struct IOProvider {
    pub gpio_driver: Box<dyn DrivesGpio>,
    pub uart_driver: Box<dyn DrivesUart>,
    pub pwm_driver: Box<dyn DrivesPwm>,
}

pub mod gpio;
pub mod net;
pub mod pwm;
pub mod uart;
