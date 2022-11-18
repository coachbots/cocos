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
            pin_in1: 35u8,
            pin_in2: 36u8,
            pin_pwm: 37u8,
            pin_stdby: 38u8
        },
        mot_right: MotorDescriptor {
            pin_in1: 29u8,
            pin_in2: 31u8,
            pin_pwm: 32u8,
            pin_stdby: 38u8
        },
        led: LedDescriptor {
            pin_r_bcm: 15u8,
            pin_g_bcm: 16u8,
            pin_b_bcm: 18u8,
            frequency: Frequency::new::<hertz>(120f32)
        },
        nucifera: NuciferaDescriptor {
            pin_uart_tx_bcm: 14u8,
            pin_uart_rx_bcm: 15u8,
            baud_rate: 19200u16
        },
    };
}
