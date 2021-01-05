use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub complete: bool,
    pub created_at: NaiveDateTime
}