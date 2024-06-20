use crate::schema::tasks;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
