extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewWatchtower, Watchtower};

pub fn create_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_watchtower(
    conn: &mut MysqlConnection,
    tower_id: &str,
    host: &str,
    port: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::watchtowers;

    let new_watchtower = NewWatchtower {
        tower_id,
        host,
        port,
    };

    diesel::insert_into(watchtowers::table)
        .values(&new_watchtower)
        .execute(conn)?;

    Ok(())
}

pub fn list_watchtowers() -> Vec<Watchtower> {
    use self::schema::watchtowers::dsl::*;

    let connection = &mut create_connection();
    let results = watchtowers
        .select(Watchtower::as_select())
        .load(connection)
        .expect("Error loading watchtowers");

    println!("Displaying {} watchtowers", results.len());
    for watchtower in &results {
        println!("{}", watchtower.host);
        println!("-----------\n");
        println!("{}", watchtower.port);
    }

    results
}
