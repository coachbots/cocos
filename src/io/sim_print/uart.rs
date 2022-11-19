use super::super::interface::uart::{DrivesUart, UartError};

#[derive(Copy, Clone)]
pub struct PrintUartDriver {}

impl PrintUartDriver {
    pub fn new() -> PrintUartDriver {
        PrintUartDriver {}
    }
}

impl DrivesUart for PrintUartDriver {
    fn read_bytes(&mut self, into: &mut [u8]) -> Result<Box<[u8]>, UartError> {
        panic!("Not implemented") // TODO
    }
}
