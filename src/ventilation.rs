use crate::relay_ctrl;
use crate::relay_ctrl::RELAY_IN3_PIN_VENTILATOR_OR_HEATER;
use std::thread;
use std::time::Duration;

const ONE_HOURS: u64 = 3600;

fn trigger_ventilator() {
    println!("trigger_ventilator() -> turning ON ventilator for 30 seconds !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, true)
        .expect("unable to change relay");
    thread::sleep(Duration::from_secs(30));
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false)
        .expect("unable to change relay");
}

pub fn ventilation_loop() {
    loop {
        trigger_ventilator();
        thread::sleep(Duration::from_secs(ONE_HOURS));
    }
}
