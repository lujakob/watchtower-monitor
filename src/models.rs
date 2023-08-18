use super::schema::watchtowers;
use diesel::Insertable;
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
pub struct Watchtower {
    pub id: i32,
    pub tower_id: String,
    pub host: String,
    pub port: i32,
}

#[derive(Insertable)]
#[table_name = "watchtowers"]
pub struct NewWatchtower<'a> {
    pub tower_id: &'a str,
    pub host: &'a str,
    pub port: i32,
}

#[derive(Serialize)]
pub struct ApiResult<T> {
    pub meta: Meta,
    pub data: Vec<T>,
}

#[derive(Serialize)]
pub struct Meta {
    pub total_entries: usize,
}
