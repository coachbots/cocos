pub enum IOError {
    Unknown,
    Reinitialization,
    Uninitialized
}

pub trait IODriver {
    /// Called upon driver initialization.
    fn init(&mut self) -> Result<(), IOError>;
}

pub mod gpio;
pub mod pwm;
