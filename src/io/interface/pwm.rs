use uom::si::f32::Frequency;

pub enum PwmError {
    IO
}

pub trait DrivesPwm {
    /// Sets the frequency and duty cycle of a specific pin.
    fn set_freq_dc(&mut self, frequency: Frequency, duty_cycle: f32,
                   pin_bcm: u8) -> Result<(), PwmError>;
}
