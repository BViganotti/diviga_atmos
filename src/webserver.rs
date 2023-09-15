use crate::AccessSharedData;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

pub fn start_webserver(sd: &AccessSharedData) {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, sd);
    }
}

fn create_not_ok_response_string() -> String {
    let res: String = "Something went wrong".to_owned();
    res
}

fn create_ok_response_string(sd: &AccessSharedData) -> String {
    let mut res: String = "<h1>last reading time: ".to_owned();
    res.push_str(&sd.last_reading_datetime().to_string());
    res.push_str("</h1><h1>temp1: ");
    res.push_str(&sd.temp_one().to_string());
    res.push_str(" - humidity1: ");
    res.push_str(&sd.humidity_one().to_string());
    res.push_str("</h1><h1>temp2: ");
    res.push_str(&sd.temp_two().to_string());
    res.push_str(" - humidity2: ");
    res.push_str(&sd.humidity_two().to_string());
    res.push_str("</h1><h1>average temp: ");
    res.push_str(&sd.average_temp().to_string());
    res.push_str("</h1><h1>average humidity: ");
    res.push_str(&sd.average_humidity().to_string());
    res.push_str("</h1><h1>fridge status: ");
    res.push_str(&sd.fridge_status().to_string());
    res.push_str("</h1><h1>humidifier status: ");
    res.push_str(&sd.humidifier_status().to_string());
    res.push_str("</h1><h1>dehumidifier status: ");
    res.push_str(&sd.dehumidifier_status().to_string());
    res.push_str("</h1><h1>heater status: ");
    res.push_str(&sd.heater_status().to_string());

    res
}

fn handle_connection(mut stream: TcpStream, sd: &AccessSharedData) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let contents: String;
    let status_line: String;
    if buffer.starts_with(get) {
        status_line = "HTTP/1.1 200 OK".to_owned();
        contents = create_ok_response_string(sd);
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND".to_owned();
        contents = create_not_ok_response_string();
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
