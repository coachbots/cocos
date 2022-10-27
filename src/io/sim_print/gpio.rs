use super::super::interface::gpio::{DrivesGpio, PullMode, GpioError};

pub struct PrintGpioDriver {
}

impl PrintGpioDriver {
    pub fn new() -> PrintGpioDriver {
        PrintGpioDriver {
        }
    }
}

impl DrivesGpio for PrintGpioDriver {
    fn set(&self, pin_bcm: u8) -> Result<(), GpioError> {
        println!("IO GPIO: Set {:?} 1", pin_bcm);
        Result::Ok(())
    }

    fn clear(&self, pin_bcm: u8) -> Result<(), GpioError> {
        println!("IO GPIO: Set {:?} 0", pin_bcm);
        Result::Ok(())
    }

    fn set_out(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), GpioError> {
        println!("IO GPIO: Config {:?} Out {:?}", pin_bcm, pull_mode);
        Result::Ok(())
    }

    fn set_inp(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), GpioError> {
        println!("IO GPIO: Config {:?} Inp {:?}", pin_bcm, pull_mode);
        Result::Ok(())
    }
}
