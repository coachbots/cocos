use std::{rc::Rc, cell::RefCell};

use crate::{
    io::interface::{uart::DrivesUart, IODriver, IOError},
    models::position::Position
};

pub trait DrivesNucifera {
    /// Reads the current position of the coachbot.
    ///
    /// In case Nucifera provides more than 1 position sample since the last
    /// invokation of this function, it is left to the implementation to decide
    /// on the behavior.
    fn read_current_position(&mut self) -> Position;
}

#[derive(Clone, Copy)]
pub struct NuciferaDescriptor {
    pub pin_uart_tx_bcm: u8,
    pub pin_uart_rx_bcm: u8,
    pub baud_rate: u16
}

pub struct NuciferaDriver {
    descriptor: NuciferaDescriptor,
    uart_driver: Rc<RefCell<dyn DrivesUart>>
}

pub struct NuciferaMessage {
}

impl IODriver for NuciferaDriver {
    fn init(&mut self) -> Result<(), IOError> {
        let mut gpio_driver = self.uart_driver.borrow_mut();
        // TODO: Call initialization steps.
        Ok(())
    }
}

impl NuciferaDriver {
    pub fn new(descriptor: NuciferaDescriptor,
               uart_driver: Rc<RefCell<dyn DrivesUart>>) -> Self {
        Self {
            descriptor,
            uart_driver
        }
    }
}

impl DrivesNucifera for NuciferaDriver {
    fn read_current_position(&mut self) -> Position {
        // TODO: Implement this
        Position::zero()
    }
}
