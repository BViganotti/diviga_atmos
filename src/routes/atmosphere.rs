use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AvgAtmosphereData {
    temperature: f32,
    humidity: f32,
}

pub async fn get_atmosphere(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let temp = sd.average_temp();
    let humi = sd.average_humidity();

    let values = AvgAtmosphereData {
        temperature: temp,
        humidity: humi,
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}
