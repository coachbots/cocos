use super::IOError;

pub enum PullMode {
    Up,
    Down,
    Floating
}

pub trait DrivesGpio {
    /// Sets the gpio value to be high.
    fn set(&mut self, pin_bcm: u8) -> Result<(), IOError>;

    /// Sets the gpio value to be low.
    fn clear(&mut self, pin_bcm: u8) -> Result<(), IOError>;

    /// Sets a pin to be an output pin.
    fn set_out(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError>;

    /// Sets a pin to be an input pin.
    fn set_inp(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError>;
}