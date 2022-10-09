use super::super::interface::uart::{DrivesUart, UartError};

#[derive(Copy, Clone)]
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

    fn write_bytes(&self, value: &[u8]) -> Result<(), UartError> {
        panic!("Not implemented") // TODO
    }

    fn read_bytes(&self, count: usize) -> Result<Box<[u8]>, UartError> {
        panic!("Not implemented") // TODO
    }
}
