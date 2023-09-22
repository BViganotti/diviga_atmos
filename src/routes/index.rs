use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};

pub async fn index(common_data: web::Data<AccessSharedData>) -> HttpResponse {
    let mut res: String = "<html><body><h1>last reading time: ".to_owned();
    res.push_str(&common_data.last_reading_datetime().to_string());
    res.push_str("</h1><h1>temp1: ");
    res.push_str(&common_data.temp_one().to_string());
    res.push_str(" - humidity1: ");
    res.push_str(&common_data.humidity_one().to_string());
    res.push_str("</h1><h1>temp2: ");
    res.push_str(&common_data.temp_two().to_string());
    res.push_str(" - humidity2: ");
    res.push_str(&common_data.humidity_two().to_string());
    res.push_str("</h1><h1>average temp: ");
    res.push_str(&common_data.average_temp().to_string());
    res.push_str("</h1><h1>average humidity: ");
    res.push_str(&common_data.average_humidity().to_string());
    res.push_str("</h1><h1>fridge status: ");
    res.push_str(&common_data.fridge_status().to_string());
    res.push_str("</h1><h1>humidifier status: ");
    res.push_str(&common_data.humidifier_status().to_string());
    res.push_str("</h1><h1>dehumidifier status: ");
    res.push_str(&common_data.dehumidifier_status().to_string());
    res.push_str("</h1><h1>heater status: ");
    res.push_str(&common_data.heater_status().to_string());
    res.push_str("</body></html>");

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(res)
}
