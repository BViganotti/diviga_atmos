use diesel::prelude::*;
use crate::schema::atmos;

#[derive(Queryable, Selectable)]
#[diesel(table_name = atmos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MyAtmos {
    pub id: i32,
    pub last_reading_time: String,
    pub average_temperature: String,
    pub average_humidity: String,
    pub temperature_one: String,
    pub humidity_one: String,
    pub temperature_two: String,
    pub humidity_two: String,
    pub humidifier_relay_state: String,
    pub last_humidifier_turn_on: String,
    pub last_humidifier_turn_off: String,
    pub frige_relay_state: String,
    pub last_fridge_turn_on: String,
    pub last_fridge_turn_off: String,
    pub dehumidifier_relay_state: String,
    pub last_dehumidifier_turn_on: String,
    pub last_dehumidifier_turn_off: String,
    pub heater_relay_state: String,
    pub last_heater_turn_on: String,
    pub last_heater_turn_off: String,
    pub atmospheric_status: String,
    pub alert: String,
    pub humidifier_water_level: String,
    pub dehumidifier_water_level: String,
}

#[derive(Insertable)]
#[diesel(table_name = atmos)]
pub struct NewAtmos<'a> {
    pub last_reading_time: &'a str,
    pub average_temperature: &'a str,
    pub average_humidity: &'a str,
}