use crate::{
    io::interface::{uart::{DrivesUart, UartParity, UartDescriptor}, IODriver, IOError},
    models::position::Position,
};
use std::sync::MutexGuard;

#[derive(Clone, Copy)]
pub struct NuciferaDescriptor {
    pub baud_rate: u32,
    pub parity: UartParity,
    pub data_bits: u8,
    pub stop_bits: u8
}

impl NuciferaDescriptor {
    pub fn to_uart_descriptor(&self) -> UartDescriptor {
        UartDescriptor {
            baud_rate: self.baud_rate,
            parity: self.parity,
            data_bits: self.data_bits,
            stop_bits: self.stop_bits
        }
    }
}

#[derive(Clone, Copy)]
pub struct NuciferaDriver {
    descriptor: NuciferaDescriptor,
}

pub struct NuciferaMessage {}

impl IODriver for NuciferaDriver {
    fn init(&mut self) -> Result<(), IOError> {
        // TODO: Call initialization steps.
        Ok(())
    }
}

impl NuciferaDriver {
    pub fn new(descriptor: NuciferaDescriptor) -> Self {
        Self { descriptor }
    }

    /// Reads the current position of the coachbot.
    ///
    /// In case Nucifera provides more than 1 position sample since the last
    /// invokation of this function, it is left to the implementation to decide
    /// on the behavior.
    pub fn read_current_position<UartDriver: DrivesUart>(&self, uart_driver:
                                                         &MutexGuard<UartDriver>) -> Position {
        // TODO: Implement this
        Position::zero()
    }
}
