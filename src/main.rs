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

    // get_todo();

    // update_todo();

    // get_todo();

    destroy_todo();
    
    Ok(())
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
    static ref DBPOOL: DbPool = {
        loop {
            dotenv().ok();

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

pub fn establish_connection() -> Result<impl Connection, Box<dyn std::error::Error>> {
}

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

fn create_todo(t: &String) -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = DBPOOL.clone().get().unwrap();
    let inserted_row = diesel::insert_into(todos)
        .values((&task.eq(t), complete.eq(false)))
        .execute(&connection)?;
    Ok(())
}

fn get_todo() -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let data = todos.select((id, task, complete)).load::<(i32, String, bool)>(&connection)?;
    for item in data {
        println!("id {} \ntask {} \n complete {}", item.0, item.1, item.2);
    }
    Ok(())
}

fn get_todos() -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let data = todos.select((task, complete)).load::<(String, bool)>(&connection)?;
    for item in data {
        println!("task {} \n complete {}", item.0, item.1);
    }
    Ok(())
}

fn update_todo() {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let data = diesel::update(todos.filter(id.eq(1)))
        .set((task.eq("Unko"), complete.eq(true)))
        .execute(&connection);
    if let Err(e) = data {
        println!("{:#?}", e);
    }
}

fn destroy_todo() -> Result<(), Box<dyn std::error::Error>> {
    use schema::todos::dsl::*;
    let connection = establish_connection();
    let len = todos.count().get_result::<i64>(&connection)?;
    println!("{:#?}", len);
    let deleted_row = diesel::delete(todos.filter(id.eq(len as i32))).execute(&connection);
    Ok(())
}

//---------------------------------------------------------