use uom::si::f32::Frequency;
use super::super::interface::pwm::{DrivesPwm, PwmError};

pub struct PrintPwmDriver {
}

impl PrintPwmDriver {
    pub fn new() -> PrintPwmDriver {
        PrintPwmDriver {
        }
    }
}

impl DrivesPwm for PrintPwmDriver {
    fn start(&self) -> Result<(), PwmError> {
        println!("IO PWM: Start");
        Ok(())
    }

    fn stop(&self) -> Result<(), PwmError> {
        println!("IO PWM: Stop");
        Ok(())
    }

    fn set_dc(&self, value: f32) -> Result<(), PwmError> {
        println!("IO PWM: Duty Cycle Set {:?}", value);
        Ok(())
    }

    fn set_freq(&self, frequency: Frequency) -> Result<(), PwmError> {
        println!("IO PWM: Frequency Set: {:?}", frequency);
        Ok(())
    }
}
