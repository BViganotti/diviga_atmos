use crate::shared_data::AccessSharedData;
use rppal::uart::Uart;
use time::OffsetDateTime;

pub fn read_atmosphere_from_sensors(mut port_reader: Uart, sd: &AccessSharedData) -> ! {
    let mut recv_buffer = [0u8; 1];
    let mut data: Vec<u8> = Vec::with_capacity(23);
    loop {
        // Fill the buffer variable with any incoming data.
        if port_reader.read(&mut recv_buffer).unwrap() > 0 {
            data.push(recv_buffer[0]);
        }
        if data.len() == 23 {
            process_data(&data, &sd);
            data.clear();
        }
    }
}

fn process_data(data: &Vec<u8>, sd: &AccessSharedData) {
    let str_data = String::from_utf8(data.to_vec()).expect("Found invalid UTF-8");

    println!("{}", str_data);

    let parts: Vec<String> = str_data.split(":").map(|s| s.to_string()).collect();

    let t1: f32 = parts[0].parse().unwrap();
    let h1: f32 = parts[1].parse().unwrap();
    let t2: f32 = parts[2].parse().unwrap();
    let h2: f32 = parts[3].parse().unwrap();
    let now = OffsetDateTime::now_utc();

    sd.increment_polling_iterations();
    sd.set_temp_one(t1);
    sd.set_humidity_one(h1);
    sd.set_temp_two(t2);
    sd.set_humidity_two(h2);
    sd.set_last_reading_datetime(now);
}
