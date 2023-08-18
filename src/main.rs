use watchtower_monitor::{
    list_watchtowers,
    models::{ApiResult, Meta, Watchtower},
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
    let result = list_watchtowers();
    let total_entries = result.len();
    Json(ApiResult {
        data: result,
        meta: Meta { total_entries },
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, watchtowers_list])
}
