extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::process::Command;

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

pub fn get_watchtowers() -> Vec<Watchtower> {
    use self::schema::watchtowers::dsl::*;

    let connection = &mut create_connection();
    let results = watchtowers
        .select(Watchtower::as_select())
        .load(connection)
        .expect("Error loading watchtowers");

    results
}

pub fn ping_watchtowers() {
    println!("ping watchtwowers");

    let list = get_watchtowers();

    for watchtower in &list {
        println!("Host: {}\n", watchtower.host);
        println!("Port: {}", watchtower.port);
        println!("-----------\n");
    }

    for entry in list {
        let mut curl_command = Command::new("curl");
        curl_command
            .arg("-s") // Silent mode to suppress progress meter
            .arg("-i") // HEAD request to fetch headers only
            .arg("-w")
            .arg("%{http_code}") // Custom output format for status code
            .arg("-o")
            .arg("/dev/null") // Discard response body
            .arg(format!("{}:{}/ping", entry.host, entry.port));

        let command_string = format!("{:?}", curl_command);

        println!("Executing curl command: {}", command_string);

        let output = curl_command.output().expect("Failed to execute command");

        if output.status.success() {
            let status_code = String::from_utf8_lossy(&output.stdout);
            println!("Host: {}, Status Code: {}", entry.host, status_code);
        } else {
            println!("Failed to ping host: {}", entry.host);
        }
    }
}
