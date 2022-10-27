use super::super::interface::uart::{DrivesUart, UartError};

#[derive(Copy, Clone)]
pub struct PrintUartDriver {
}

impl PrintUartDriver {
    pub fn new() -> PrintUartDriver {
        PrintUartDriver {
        }
    }
}

impl DrivesUart for PrintUartDriver {
    fn read_byte(&self) -> Result<u8, UartError> {
        println!("IO UART: Read {:?}", 0);
        Ok(0)
    }

    fn write_byte(&self, value: u8) -> Result<(), UartError> {
        println!("IO UART: Write {:?}", value);
        Ok(())
    }

    fn write_bytes(&self, value: &[u8]) -> Result<(), UartError> {
        println!("IO UART: Write+ {:?}", value);
        Ok(())
    }

    fn read_bytes(&self, count: usize) -> Result<Box<[u8]>, UartError> {
        let ret_val = Box::new([1u8, 2u8, 3u8]);
        println!("IO UART: Read+ N={:?} {:?}", count, ret_val);
        Ok(ret_val)
    }
}
