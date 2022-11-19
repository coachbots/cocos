/// Represents possible UART errors.
pub enum UartError {
    IO
}

#[derive(Clone, Copy)]
/// Represents the possible parity modes for the IO UART layer.
pub enum UartParity { None, Even, Odd, Mark, Space }

#[derive(Clone, Copy)]
/// Represents the information required to construct an IO UART controller.
pub struct UartDescriptor {
    pub baud_rate: u32,
    pub parity: UartParity,
    pub data_bits: u8,
    pub stop_bits: u8
}

pub trait DrivesUart {
    fn read_bytes(&mut self, into: &mut [u8]) -> Result<Box<[u8]>, UartError>;
}
