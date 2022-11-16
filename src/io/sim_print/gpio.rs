use std::{fs::File, io::Write, time::Instant};

use super::super::interface::gpio::{DrivesGpio, GpioError, PullMode};

/// This GPIO IO implementation outputs its data to an injected file. The
/// encoding is a tremendously simple CSV-style output that can be used to
/// fully reconstruct the outputs.
///
/// The message always appears in one single line and may look like this:
/// * `S,<PIN: int>,<STATE: {0/1}>`
/// * `C,<PIN: int>,<DIRECTION: {I/O}>,<PULL_MODE: {D/U}>`
///
/// A `C` packet indicates a GPIO configuration while an `S` packet represents
/// an output state change.
pub struct PrintGpioDriver {
    file: File,
    init_time: Instant
}

impl PrintGpioDriver {
    pub fn new(file: File, init_time: Instant) -> PrintGpioDriver {
        PrintGpioDriver { file, init_time }
    }
}

impl DrivesGpio for PrintGpioDriver {
    fn set(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        let time_since_init = Instant::now() - self.init_time;
        writeln!(self.file, "{:},1,S,{:},1", time_since_init.as_secs_f64(), pin_bcm);
        Result::Ok(())
    }

    fn clear(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        let time_since_init = Instant::now() - self.init_time;
        writeln!(self.file, "{:},1,S,{:},0", time_since_init.as_secs_f64(), pin_bcm);
        Result::Ok(())
    }

    fn set_out(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError> {
        let time_since_init = Instant::now() - self.init_time;
        writeln!(self.file, "{:},1,C,{:},O,{:?}", time_since_init.as_secs_f64(), pin_bcm, pull_mode);
        Result::Ok(())
    }

    fn set_inp(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError> {
        let time_since_init = Instant::now() - self.init_time;
        writeln!(self.file, "{:},1,C,{:},I,{:?}", time_since_init.as_secs_f64(), pin_bcm, pull_mode);
        Result::Ok(())
    }
}
