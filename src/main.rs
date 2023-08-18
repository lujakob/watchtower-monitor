use watchtower_monitor::{
    get_watchtowers,
    models::{ApiResult, Meta, Watchtower},
    ping_watchtowers,
};

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/watchtowers/list")]
fn watchtowers_list() -> Json<ApiResult<Watchtower>> {
    let result = get_watchtowers();
    let total_entries = result.len();
    Json(ApiResult {
        data: result,
        meta: Meta { total_entries },
    })
}

#[get("/watchtowers/ping")]
fn watchtowers_ping() {
    ping_watchtowers();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, watchtowers_list, watchtowers_ping])
}
