#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use models::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let results = todos
        .limit(5)
        .load::<Todo>(&connection)?;

    for result in results {

    }
    
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}