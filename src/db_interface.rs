use diesel::sqlite::Sqlite;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::*;
use crate::schema::atmos;
use crate::schema::atmos::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_new_atmos_data(conn: &mut SqliteConnection, last_read: &str, avg_temp: &str, avg_humi: &str) {

    let new_atmos_data = NewAtmos { last_reading_time: last_read, average_temperature: avg_temp, average_humidity: avg_humi };

    let rows_inserted = diesel::insert_into(atmos::table)
        .values(&new_atmos_data)
        .execute(conn);
    assert_eq!(Ok(1), rows_inserted);
}