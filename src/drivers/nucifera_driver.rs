use std::{rc::Rc, cell::RefCell};

use crate::{
    io::interface::{uart::DrivesUart, IODriver, IOError},
    models::position::Position
};

#[derive(Clone, Copy)]
pub struct NuciferaDescriptor {
    pub pin_uart_tx_bcm: u8,
    pub pin_uart_rx_bcm: u8,
    pub baud_rate: u16
}

#[derive(Clone, Copy)]
pub struct NuciferaDriver {
    descriptor: NuciferaDescriptor
}

pub struct NuciferaMessage {
}

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
    pub fn read_current_position(&self,
                                 uart_driver: &impl DrivesUart) -> Position {
        // TODO: Implement this
        Position::zero()
    }
}
