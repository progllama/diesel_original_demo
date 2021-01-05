use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub complete: bool,
    pub published_at: Timestamp,
}