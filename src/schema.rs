// @generated automatically by Diesel CLI.

diesel::table! {
    atmosphere (id) {
        id -> Nullable<Integer>,
        last_reading_time -> Nullable<Text>,
        average_temperature -> Nullable<Text>,
        average_humidity -> Nullable<Text>,
        temperature_one -> Nullable<Text>,
        humidity_one -> Nullable<Text>,
        temperature_two -> Nullable<Text>,
        humidity_two -> Nullable<Text>,
        humidifier_relay_state -> Nullable<Text>,
        last_humidifier_turn_on -> Nullable<Text>,
        last_humidifier_turn_off -> Nullable<Text>,
        frige_relay_state -> Nullable<Text>,
        last_fridge_turn_on -> Nullable<Text>,
        last_fridge_turn_off -> Nullable<Text>,
        dehumidifier_relay_state -> Nullable<Text>,
        last_dehumidifier_turn_on -> Nullable<Text>,
        last_dehumidifier_turn_off -> Nullable<Text>,
        heater_relay_state -> Nullable<Text>,
        last_heater_turn_on -> Nullable<Text>,
        last_heater_turn_off -> Nullable<Text>,
        atmospheric_status -> Nullable<Text>,
        alert -> Nullable<Text>,
        humidifier_water_level -> Nullable<Text>,
        dehumidifier_water_level -> Nullable<Text>,
    }
}
