// use sqlx::{FromRow, types::chrono::{self, Utc}};
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;
// use crate::AuthUser;
// use chrono::serde::ts_seconds::serialize as to_ts;

// #[derive(FromRow)]
#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    // auth_user: AuthUser,
    pub email: String,
    pub passwd: String,
}

// #[derive(FromRow)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct History {
    pub id: i32,
    pub user_id: i32,
    pub bc_link: String,
    // #[serde(serialize_with = "to_ts")]
    pub history_date: NaiveDate,
}