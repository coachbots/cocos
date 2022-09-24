use super::super::interface::uart::{DrivesUart, UartError};

pub struct RpiUartDriver {
}

impl RpiUartDriver {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl RpiUartDriver {
}

impl DrivesUart for RpiUartDriver {
    fn read_byte(&self) -> Result<u8, UartError> {
        panic!("Not implemented") // TODO
    }

    fn write_byte(&self, value: u8) -> Result<(), UartError> {
        panic!("Not implemented") // TODO
    }
}
