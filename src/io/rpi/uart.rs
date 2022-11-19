use rppal::uart::{Uart, Parity};
use crate::io::interface::uart::{UartDescriptor, UartParity};

use super::super::interface::uart::{DrivesUart, UartError};

impl UartParity {
    fn to_rppal(&self) -> Parity {
        match self {
            UartParity::None => Parity::None,
            Even => Parity::Even,
            Odd => Parity::Odd,
            Mark => Parity::Mark,
            Space => Parity::Space
        }
    }
}

pub struct RpiUartDriver {
    rpi_driver: Uart,
}

impl RpiUartDriver {
    pub fn new(uart_descriptor: UartDescriptor) -> Self {
        Self {
            rpi_driver: Uart::new(uart_descriptor.baud_rate, uart_descriptor.parity.to_rppal(),
                                  uart_descriptor.data_bits, uart_descriptor.stop_bits).expect(
                                      "Could not initialize the rppal Uart driver")
        }
    }
}

impl DrivesUart for RpiUartDriver {
    fn read_bytes(&mut self, into: &mut [u8]) -> Result<Box<[u8]>, UartError> {
        panic!("Not implemented") // TODO
    }
}
