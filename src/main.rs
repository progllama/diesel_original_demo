#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub mod schema;
pub mod models;

use models::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    {
        use schema::todos::dsl::*;
        let connection = establish_connection();
        
        diesel::insert_into(todos)
            .values((&task.eq("UNK"), &complete.eq(false)))
            .execute(&connection)?;
        let results = todos
            .limit(20)
            .load::<Todo>(&connection)?;

        for result in results {
            println!("{:#?}", result);
        }
    }

    let mut tsk = String::new();
    std::io::stdin().read_line(&mut tsk);

    create_todo(&tsk);
    
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

//---------------------------------------------------------
// Http server function
//---------------------------------------------------------

//---------------------------------------------------------

//---------------------------------------------------------
// DB function
//---------------------------------------------------------

//prepare DBPool

lazy_static! {
    static ref DbPOOL: DbPool = {
        loop {
            let connspec = if let Ok(v) = std::env::var("DATABASE_URL") {
                v
            } else {
                continue;
            };
            let manager = ConnectionManager::<PgConnection>::new(connspec);
            let pool = r2d2::Pool::builder()
                .build(manager);
            if let Ok(p) = pool {
                break p;
            }
        }
    };
}

fn create_todo(t: &String) -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let inserted_row = diesel::insert_into(todos)
        .values((&task.eq(t), complete.eq(false)))
        .execute(&connection)?;
    Ok(())
}

fn get_todo() {

}

fn get_todos() {

}

fn update_todo() {

}

fn destroy_todo() {

}

//---------------------------------------------------------