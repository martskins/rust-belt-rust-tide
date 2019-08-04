use serde::*;

use crate::schema::user;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
