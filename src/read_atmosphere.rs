use crate::shared_data::AccessSharedData;
use rppal::uart::Uart;
use serde_json::Value;
use std::process::Command;
use time::macros::offset;
use time::OffsetDateTime;

pub fn read_atmosphere_from_sensors(sd: &AccessSharedData) {
    let output = Command::new("python3")
        .arg("test.py")
        .output()
        .expect("failed to execute process");

    let str_output: String = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", str_output);

    let v: Value = serde_json::from_str(&str_output).unwrap();

    println!(
        "temp1: {} humi1: {}temp2: {} humi2: {}",
        v["t1"], v["h1"], v["t2"], v["h2"]
    );

    let t1: f32 = v["t1"].as_f64().unwrap() as f32;
    let h1: f32 = v["h1"].as_f64().unwrap() as f32;
    let t2: f32 = v["t2"].as_f64().unwrap() as f32;
    let h2: f32 = v["h2"].as_f64().unwrap() as f32;
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));

    sd.increment_polling_iterations();
    sd.set_temp_one(t1);
    sd.set_humidity_one(h1);
    sd.set_temp_two(t2);
    sd.set_humidity_two(h2);
    sd.set_last_reading_datetime(now);
}

// pub fn read_atmosphere_from_sensors(mut port_reader: Uart, sd: &AccessSharedData) -> ! {
//     let mut recv_buffer = [0u8; 1];
//     let mut data: Vec<u8> = Vec::with_capacity(23);
//     loop {
//         // Fill the buffer variable with any incoming data.
//         if port_reader.read(&mut recv_buffer).unwrap() > 0 {
//             data.push(recv_buffer[0]);
//         }
//         if data.len() == 23 {
//             process_data(&data, &sd);
//             data.clear();
//         }
//     }
// }

// fn process_data(data: &Vec<u8>, sd: &AccessSharedData) {
//     let str_data = String::from_utf8(data.to_vec()).expect("Found invalid UTF-8");

//     println!("{}", str_data);

//     let parts: Vec<String> = str_data.split(":").map(|s| s.to_string()).collect();

//     let t1: f32 = parts[0].parse().unwrap();
//     let h1: f32 = parts[1].parse().unwrap();
//     let t2: f32 = parts[2].parse().unwrap();
//     let h2: f32 = parts[3].parse().unwrap();
//     let now = OffsetDateTime::now_utc().to_offset(offset!(+1));

//     sd.increment_polling_iterations();
//     sd.set_temp_one(t1);
//     sd.set_humidity_one(h1);
//     sd.set_temp_two(t2);
//     sd.set_humidity_two(h2);
//     sd.set_last_reading_datetime(now);
// }
