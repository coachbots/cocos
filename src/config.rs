use crate::drivers::motor_driver::MotorDescriptor;

pub struct AppConfig {
    pub mot_left: MotorDescriptor,
    pub mot_right: MotorDescriptor
}

pub static APP_CONFIG: AppConfig = AppConfig {
    mot_left: MotorDescriptor {
        pin_left_bcm: 3u8,
        pin_right_bcm: 4u8
    },
    mot_right: MotorDescriptor {
        pin_left_bcm: 3u8,
        pin_right_bcm: 4u8
    },
};
