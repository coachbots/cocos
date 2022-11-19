use uom::si::{f32::Frequency, frequency::hertz};

use crate::drivers::{
    led_driver::LedDescriptor,
    motor_driver::MotorDescriptor,
    nucifera_driver::NuciferaDescriptor,
};
use crate::io::interface::uart::UartParity;

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
            baud_rate: 19200u32,
            parity: UartParity::Even,
            data_bits: 8,
            stop_bits: 1
        },
    };
}
