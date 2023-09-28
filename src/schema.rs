// @generated automatically by Diesel CLI.

diesel::table! {
    atmos (id) {
        id -> Integer,
        last_reading_time -> Text,
        average_temperature -> Text,
        average_humidity -> Text,
        temperature_one -> Text,
        humidity_one -> Text,
        temperature_two -> Text,
        humidity_two -> Text,
        humidifier_relay_state -> Text,
        last_humidifier_turn_on -> Text,
        last_humidifier_turn_off -> Text,
        frige_relay_state -> Text,
        last_fridge_turn_on -> Text,
        last_fridge_turn_off -> Text,
        dehumidifier_relay_state -> Text,
        last_dehumidifier_turn_on -> Text,
        last_dehumidifier_turn_off -> Text,
        heater_relay_state -> Text,
        last_heater_turn_on -> Text,
        last_heater_turn_off -> Text,
        atmospheric_status -> Text,
        alert -> Text,
        humidifier_water_level -> Text,
        dehumidifier_water_level -> Text,
    }
}
