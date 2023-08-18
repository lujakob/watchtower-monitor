extern crate diesel;

use watchtower_monitor::{create_connection, create_watchtower};

fn main() {
    let mut connection = create_connection();
    let tower_id: &str = "03b0a47228c8b36477002d745cf578a4db2b7c2ba2e2aee2c65419053e4375a1dd";
    let host = "satseye.slicksparks.ky";
    let port: i32 = 9814;

    create_watchtower(&mut connection, tower_id, host, port)
        .unwrap_or_else(|e| eprint!("insert error: {e}"));
}
