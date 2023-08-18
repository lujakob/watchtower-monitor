// @generated automatically by Diesel CLI.

diesel::table! {
    watchtowers (id) {
        id -> Integer,
        #[max_length = 255]
        tower_id -> Varchar,
        #[max_length = 255]
        host -> Varchar,
        port -> Integer,
    }
}
