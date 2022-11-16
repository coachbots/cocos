use std::{fs::File, io::Write, time::{Instant, Duration}, thread::{self, JoinHandle}, collections::HashMap, borrow::Borrow, sync::{Arc, atomic::AtomicBool, Mutex}};

use super::super::interface::pwm::{DrivesPwm, PwmError};
use uom::si::f32::Frequency;

pub struct PrintPwmDriver {
    file: Arc<Mutex<File>>,
    begin_time: Instant,
    pwm_pin_map: HashMap<u8, (JoinHandle<()>, Arc<Mutex<(Frequency, f32)>>)>
}

impl PrintPwmDriver {
    pub fn new(file: File, begin_time: Instant) -> PrintPwmDriver {
        let driver = PrintPwmDriver {
            file: Arc::new(Mutex::new(file)),
            begin_time,
            pwm_pin_map: HashMap::new()
        };
        driver
    }
}

/// Due to the fact that the PWM used on the raspberry pi is a software PWM, this emulates that
/// behavior via threads. Shutdown is ungraceful, but who cares?
impl DrivesPwm for PrintPwmDriver {
    fn set_freq_dc(
        &mut self,
        frequency: Frequency,
        duty_cycle: f32,
        pin_bcm: u8,
    ) -> Result<(), PwmError> {
        if !self.pwm_pin_map.contains_key(&pin_bcm) {
            self.pwm_pin_map.insert(pin_bcm, (
                thread::spawn(||{}),
                Arc::new(Mutex::new((frequency, duty_cycle)))
            ));

            let file = Arc::clone(&self.file);
            let begin_time = self.begin_time;
            let data_mutex = Arc::clone(&self.pwm_pin_map[&pin_bcm].1);

            let work_thread = thread::spawn(move || {
                loop {
                    let (frequency, duty_cycle) = data_mutex.lock().unwrap().clone();

                    let time_since_start = Instant::now() - begin_time;
                    let period = 1f32 / frequency.value;
                    writeln!(file.lock().unwrap(), "{:},1,S,{:},1", time_since_start.as_secs_f64(),
                             pin_bcm);
                    thread::sleep(Duration::from_secs_f32(duty_cycle * period));
                    writeln!(file.lock().unwrap(), "{:},1,S,{:},0", time_since_start.as_secs_f64(),
                             pin_bcm);
                    thread::sleep(Duration::from_secs_f32((1f32 - duty_cycle) * period));
                }
            });

            self.pwm_pin_map.get_mut(&pin_bcm).unwrap().0 = work_thread;
        } else {
            let pin_data_arc = self.pwm_pin_map[&pin_bcm].1.clone();
            let mut pin_data = pin_data_arc.lock().unwrap();
            *pin_data = (frequency, duty_cycle);
        }
        Ok(())
    }
}
