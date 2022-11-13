pub enum UartError {}

pub trait DrivesUart {
    /// Attempts to read a byte from the UART peripheral.
    fn read_byte(&self) -> Result<u8, UartError>;

    /// Attempts to write a byte to the UART peripheral.
    fn write_byte(&self, value: u8) -> Result<(), UartError>;

    fn write_bytes(&self, value: &[u8]) -> Result<(), UartError>;

    fn read_bytes(&self, count: usize) -> Result<Box<[u8]>, UartError>;
}
