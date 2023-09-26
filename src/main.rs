pub mod monitor_atmosphere;
pub mod read_atmosphere;
pub mod relay_ctrl;
pub mod request_atmosphere;
pub mod routes;
pub mod shared_data;
pub mod webserver;

use rppal::uart::{Parity, Uart};
use std::thread;
use std::time::Duration;

use crate::shared_data::AccessSharedData;
use crate::shared_data::SharedData;
use actix_web::rt;
use std::sync::Arc;
use std::sync::Mutex;
use time::OffsetDateTime;
use dotenv::dotenv;

/*
This project is very very much influenced by https://github.com/mikehentges/thermostat-pi.
*/

fn main() {

    dotenv().ok();

    let port_writer = Uart::with_path("/dev/ttyUSB0", 9_600, Parity::None, 8, 1).unwrap();
    let mut port_reader = Uart::with_path("/dev/ttyUSB0", 9_600, Parity::None, 8, 1).unwrap();

    port_reader.set_read_mode(1, Duration::default()).unwrap();

    // Initialize a struct that will be our "global" data, which allows safe access from every thread
    let common_data = SharedData::new(
        true,
        0.0,
        80.0,
        0.0,
        80.0,
        0.0,
        80.0,
        0.0,
        false,
        false,
        false,
        false,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
        OffsetDateTime::UNIX_EPOCH,
    );

    // The wrapper around our shared data that gives it safe access across threads
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    // We are cloning the pointer to our shared data, and sending it into
    // a new thread that continuously reads the temperature from our sensor,
    // and updates the SharedData::current_temp value.
    let sdc = sd.clone();

    let handle = thread::spawn(move || {
        request_atmosphere::request_atmosphere(port_writer);
    });

    thread::spawn(move || {
        read_atmosphere::read_atmosphere_from_sensors(port_reader, &sdc);
    });

    let sdccc = sd.clone();
    thread::spawn(move || {
        monitor_atmosphere::atmosphere_monitoring(&sdccc);
    });

    let ccccc = sd.clone();
    thread::spawn(move || {
        let server_future = webserver::run_app(&ccccc);
        rt::System::new().block_on(server_future)
    });

    let _ = handle.join();
}
