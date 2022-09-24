use std::cell::RefCell;
use std::rc::Rc;

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

pub struct IOProvider {
    pub gpio_driver: Rc<RefCell<dyn DrivesGpio>>,
    pub uart_driver: Rc<RefCell<dyn DrivesUart>>,
    pub pwm_driver: Rc<RefCell<dyn DrivesPwm>>
}

pub mod gpio;
pub mod pwm;
pub mod uart;
