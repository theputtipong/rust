use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}
