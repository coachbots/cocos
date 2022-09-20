use super::IOError;

pub trait DrivesUart {
    /// Attempts to read a byte from the UART peripheral.
    fn read_byte(&self) -> Result<u8, IOError>;

    /// Attempts to write a byte to the UART peripheral.
    fn write_byte(&self, value: u8);
}
