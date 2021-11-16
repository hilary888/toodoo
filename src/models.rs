use chrono::{ DateTime, Utc };
use crate::schema::todo;

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct TodoData {
    pub title: String,
    pub body: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name="todo"]
pub struct NewTodo {
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}