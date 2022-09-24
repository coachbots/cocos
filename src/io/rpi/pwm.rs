use uom::si::f32::Frequency;
use crate::io::interface::pwm::{DrivesPwm, PwmError};

pub struct RpiPwmDriver {
}

impl RpiPwmDriver {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl DrivesPwm for RpiPwmDriver {
    fn set_dc(&self, value: f32) -> Result<(), PwmError> {
        panic!("Not Implemented") // TODO
    }

    fn set_freq(&self, frequency: Frequency) -> Result<(), PwmError> {
        panic!("Not Implemented") // TODO
    }

    fn stop(&self) -> Result<(), PwmError> {
        panic!("Not Implemented") // TODO
    }

    fn start(&self) -> Result<(), PwmError> {
        panic!("Not Implemented") // TODO
    }
}
