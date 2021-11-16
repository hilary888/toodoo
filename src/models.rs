use chrono::{ DateTime, Utc };



#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}