use std::time::Duration;

use uom::si::{f32::Frequency, frequency::hertz};

use crate::drivers::{
    led_driver::LedDescriptor, motor_driver::MotorDescriptor, nucifera_driver::NuciferaDescriptor,
};

pub struct AppConfig {
    pub mot_left: MotorDescriptor,
    pub mot_right: MotorDescriptor,
    pub nucifera: NuciferaDescriptor,
    pub led: LedDescriptor,
}

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = AppConfig {
        mot_left: MotorDescriptor {
            pin_in1: 19u8,
            pin_in2: 16u8,
            pin_pwm: 26u8,
            pin_stdby: 20u8
        },
        mot_right: MotorDescriptor {
            pin_in1: 5u8,
            pin_in2: 6u8,
            pin_pwm: 12u8,
            pin_stdby: 20u8
        },
        led: LedDescriptor {
            pin_r_bcm: 22u8,
            pin_g_bcm: 23u8,
            pin_b_bcm: 24u8,
            frequency: Frequency::new::<hertz>(120f32)
        },
        nucifera: NuciferaDescriptor {
            pin_uart_tx_bcm: 14u8,
            pin_uart_rx_bcm: 15u8,
            baud_rate: 19200u16
        },
    };
}
