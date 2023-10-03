use rppal::uart::Uart;
use std::thread;
use std::time::Duration;
pub fn request_atmosphere(mut port_writer: Uart) {
    let send_buffer = [100];

    loop {
        port_writer.write(&send_buffer).unwrap();
        thread::sleep(Duration::from_secs(20));
    }
}
