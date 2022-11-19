/// This interface exposes the GPIO IO layer.
/// This layer is responsible for setting GPIO pins high or low.

#[derive(Debug)]
/// Enumeration exposing internal pull values.
pub enum PullMode {
    Up,
    Down,
}

/// Errors this layer can possibly throw.
pub enum GpioError {
    /// Thrown upon a GPIO IO error case.
    IO,
}

pub trait DrivesGpio {
    /// Sets the gpio value to be high.
    ///
    /// Arguments:
    /// * `pin_bcm` - The BCM pin number on the Raspberry Pi
    fn set(&mut self, pin_bcm: u8) -> Result<(), GpioError>;

    /// Sets the gpio value to be low.
    ///
    /// Arguments:
    /// * `pin_bcm` - The BCM pin number on the Raspberry Pi
    fn clear(&mut self, pin_bcm: u8) -> Result<(), GpioError>;

    /// Sets a pin to be an output pin.
    ///
    /// Arguments:
    /// * `pin_bcm` - The BCM pin number on the Raspberry Pi
    /// * `pull_mode` - whether the pin should be pulled up or down.
    fn set_out(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError>;

    /// Sets a pin to be an input pin.
    ///
    /// Arguments:
    /// * `pin_bcm` - The BCM pin number on the Raspberry Pi
    /// * `pull_mode` - whether the pin should be pulled up or down.
    fn set_inp(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError>;
}
