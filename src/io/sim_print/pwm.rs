use std::{fs::File, io::Write};

use super::super::interface::pwm::{DrivesPwm, PwmError};
use uom::si::f32::Frequency;

pub struct PrintPwmDriver {
    file: File,
}

impl PrintPwmDriver {
    pub fn new(file: File) -> PrintPwmDriver {
        PrintPwmDriver { file }
    }
}

impl DrivesPwm for PrintPwmDriver {
    fn set_freq_dc(
        &mut self,
        frequency: Frequency,
        duty_cycle: f32,
        pin_bcm: u8,
    ) -> Result<(), PwmError> {
        writeln!(self.file, "P,{},{}", frequency.value, duty_cycle);
        Ok(())
    }
}
