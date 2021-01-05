use diesel::pg::data_types::PgTimestamp;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub complete: bool,
    pub published_at: PgTimestamp,
}