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

    /// Determines the maximum motor power update rate in milliseconds.
    pub drive_update_period: Duration,
}

lazy_static! {
    pub static ref APP_CONFIG: AppConfig = AppConfig {
        mot_left: MotorDescriptor {
            pin_left_bcm: 3u8,
            pin_right_bcm: 4u8
        },
        mot_right: MotorDescriptor {
            pin_left_bcm: 3u8,
            pin_right_bcm: 4u8
        },
        led: LedDescriptor {
            pin_r_bcm: 22,
            pin_g_bcm: 23,
            pin_b_bcm: 24,
            frequency: Frequency::new::<hertz>(120f32)
        },
        nucifera: NuciferaDescriptor {
            pin_uart_tx_bcm: 14u8,
            pin_uart_rx_bcm: 15u8,
            baud_rate: 19200u16
        },

        drive_update_period: Duration::from_millis(8) // 125Hz
    };
}
