use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};
use time::format_description;

fn color_status(status: bool) -> String {
    if status != true {
        "<span style=\"color: #ff0000;\">OFF</span>".to_owned()
    } else {
        "<span style=\"color: #00ff00;\">ON</span>".to_owned()
    }
}

pub async fn index(common_data: web::Data<AccessSharedData>) -> HttpResponse {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

    let mut html_res: String = format!(
        "<h2>Atmospheric data:</h2>
    <ul>    
    <li><em>Average temperature:</em> <strong>{}</strong> degrees</li>
    <li><em>Average Humidity:</em> <strong>{}</strong> %RH</li>
    </ul>
    <ul>
    <li><em>Last reading time:</em> <strong>{}</strong></li>
    <li><em>Temp 1:</em> <strong>{}</strong> degrees - <em>Humidity 1:</em> <strong>{} %RH</strong></li>
    <li><em>Temp 2:</em> <strong>{}</strong> degrees - <em>Humidity 2:</em> <strong>{} %RH</strong></li>
    </ul>
    <ul>
    <li><em>Fridge status:</em> <strong>{}</strong></li>
    <li>last fridge turn ON time: </li>
     <li><strong>{}</strong></li>
    <li>last fridge turn OFF time: </li>
     <li><strong>{}</strong></li>
    </ul>
    <ul>
    <li><em>Humidifier status:</em> <strong>{}</strong></li>
    <li>last humidifier turn ON time: </li>
     <li><strong>{}</strong></li>
    <li>last humidifier turn OFF time: </li>
     <li><strong>{}</strong></li>
    </ul>
    <ul>
    <li><em>Dehumidifier status:</em> <strong>{}</strong></li>
    <li>last dehumidifier turn ON time: </li>
     <li><strong>{}</strong></li>
    <li>last dehumidifier turn OFF time: </li>
     <li><strong>{}</strong></li>
    </ul>
    <p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p><p>&nbsp;</p>
    <p><strong>Copyright DIVIGA 2023-Eternity</strong></p></body></html>",
        &common_data.average_temp().to_string(),
        &common_data.average_humidity().to_string(),
        &common_data.last_reading_datetime().format(&format).unwrap(),
        &common_data.temp_one().to_string(),
        &common_data.humidity_one().to_string(),
        &common_data.temp_two().to_string(),
        &common_data.humidity_two().to_string(),
        color_status(common_data.fridge_status()),
        &common_data
            .fridge_turn_on_datetime()
            .format(&format)
            .unwrap(),
        &common_data
            .fridge_turn_off_datetime()
            .format(&format)
            .unwrap(),
        color_status(common_data.humidifier_status()),
        &common_data
            .humidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
        &common_data
            .humidifier_turn_off_datetime()
            .format(&format)
            .unwrap(),
        color_status(common_data.dehumidifier_status()),
        &common_data
            .dehumidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
        &common_data
            .dehumidifier_turn_off_datetime()
            .format(&format)
            .unwrap()
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_res)
}
