use crate::relay_ctrl::change_relay_status;
use crate::relay_ctrl::{RELAY_IN1_PIN, RELAY_IN2_PIN, RELAY_IN4_PIN};
use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};
use time::OffsetDateTime;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeFridgeStatus {
    previous_fridge_status: bool,
    new_fridge_status: bool,
    last_fridge_turn_on: String,
    last_fridge_turn_off: String,
    response: String,
}

pub async fn change_fridge_status(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let prev_fridge_status = sd.fridge_status();
    let mut res = String::new();
    let now = OffsetDateTime::now_utc();

    if sd.fridge_status() == true {
        if now - sd.fridge_turn_on_datetime() < time::Duration::minutes(20) {
            // we might be just entering the ideal range so we also wait 20 minutes
            // because lowering the temp takes a while
            let wait_time = now - sd.fridge_turn_on_datetime();
            let how_long_to_wait = wait_time.as_seconds_f64() * 60.0;
            res = format!(
                "fridge still on, have to wait {} minutes before turn off",
                how_long_to_wait
            );
        } else {
            // more than 20 minutes have passed since the last turn on
            // we can safely turn off the fridge
            println!("fridge_control() -> turning off the fridge !");
            change_relay_status(RELAY_IN4_PIN, false);
            sd.set_fridge_status(false);
            sd.set_fridge_turn_off_datetime(now);
            res = "fridge turned off !".to_owned();
        }
    } else if sd.fridge_status() == false {
        if now - sd.fridge_turn_off_datetime() < time::Duration::minutes(20) {
            // we might be just entering the ideal range so we also wait 20 minutes
            // because lowering the temp takes a while
            let wait_time = now - sd.fridge_turn_off_datetime();
            let how_long_to_wait = wait_time.as_seconds_f64() * 60.0;
            res = format!(
                "fridge still off, have to wait {} minutes before turn on",
                how_long_to_wait
            );
        } else {
            // more than 20 minutes have passed since the last turn on
            // we can safely turn off the fridge
            println!("fridge_control() -> turning on the fridge !");
            change_relay_status(RELAY_IN4_PIN, true);
            sd.set_fridge_status(true);
            sd.set_fridge_turn_on_datetime(now);
            res = "fridge turned on !".to_owned();
        }
    }

    let values = ChangeFridgeStatus {
        previous_fridge_status: prev_fridge_status,
        new_fridge_status: sd.fridge_status(),
        last_fridge_turn_on: sd.fridge_turn_on_datetime().to_string(),
        last_fridge_turn_off: sd.fridge_turn_off_datetime().to_string(),
        response: res.to_owned(),
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeHumidifierStatus {
    humidifier_status: bool,
    last_humidifier_turn_on: String,
    last_humidifier_turn_off: String,
    response: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeDehumidifierStatus {
    dehumidifier_status: bool,
    last_dehumidifier_turn_on: String,
    last_dehumidifier_turn_off: String,
    response: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeHeaterStatus {
    heater_status: bool,
    last_heater_turn_on: String,
    last_heater_turn_off: String,
    response: String,
}
