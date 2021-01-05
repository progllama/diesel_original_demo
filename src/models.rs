use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub complete: bool,
    pub created_at: NaiveDateTime
}