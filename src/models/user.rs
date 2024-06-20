use crate::schema::users;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
