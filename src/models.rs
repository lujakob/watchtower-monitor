use super::schema::watchtowers;
use diesel::Insertable;
use diesel::Queryable;

#[derive(Queryable)]
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
