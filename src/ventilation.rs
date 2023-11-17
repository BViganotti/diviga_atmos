use crate::relay_ctrl;
use crate::relay_ctrl::RELAY_IN3_PIN;
use std::thread;
use std::time::Duration;

const TWO_HOURS: u64 = 7200;

fn trigger_ventilator() {
    println!("trigger_ventilator() -> turning ON ventilator for 10 seconds !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN, true).expect("unable to change relay");
    thread::sleep(Duration::from_secs(10));
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN, false).expect("unable to change relay");
}

pub fn ventilation_loop() {
    loop {
        trigger_ventilator();
        thread::sleep(Duration::from_secs(TWO_HOURS));
    }
}
