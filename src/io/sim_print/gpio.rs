use std::{fs::File, io::Write};

use super::super::interface::gpio::{DrivesGpio, PullMode, GpioError};

/// This GPIO IO implementation outputs its data to an injected file. The
/// encoding is a tremendously simple CSV-style output that can be used to
/// fully reconstruct the outputs.
///
/// The message always appears in one single line and may look like this:
/// * `S,<PIN: int>,<STATE: {0/1}>`
/// * `C,<PIN: int>,<DIRECTION: {I/O}>,<PULL_MODE>`
///
/// A `C` packet indicates a GPIO configuration while an `S` packet represents
/// an output state change.
pub struct PrintGpioDriver {
    file: File
}

impl PrintGpioDriver {
    pub fn new(file: File) -> PrintGpioDriver {
        PrintGpioDriver {
            file
        }
    }
}

impl DrivesGpio for PrintGpioDriver {
    fn set(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        writeln!(self.file, "S,{:?},1", pin_bcm);
        Result::Ok(())
    }

    fn clear(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        writeln!(self.file, "S,{:?},0", pin_bcm);
        Result::Ok(())
    }

    fn set_out(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), GpioError> {
        writeln!(self.file, "C,{:?},O,{:?}", pin_bcm, pull_mode);
        Result::Ok(())
    }

    fn set_inp(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), GpioError> {
        writeln!(self.file, "C,{:?},I,{:?}", pin_bcm, pull_mode);
        Result::Ok(())
    }
}
