use uom::si::f32::Frequency;

/// An enumeration of possible errors to be thrown.
pub enum PwmError {
    /// Represents a HAL layer error.
    IO,
}

pub trait DrivesPwm {
    /// Sets the frequency and duty cycle of a specific pin.
    ///
    /// Arguments:
    /// * `frequency` - The PWM frequency.
    /// * `duty_cycle` - The PWM duty cycle ranging from 0 to 1.
    /// * `pin_bcm` - The Pi BCM pin.
    fn set_freq_dc(&mut self, frequency: Frequency, duty_cycle: f32,
                   pin_bcm: u8) -> Result<(), PwmError>;
}
