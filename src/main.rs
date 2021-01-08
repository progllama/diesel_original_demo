#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use lazy_static::lazy_static;

use actix_web::{
    HttpServer,
    App,
    middleware,
    get,
    HttpResponse,
    Result as AWResult,
};
use actix_web::http::{
    StatusCode,
};

pub mod schema;
pub mod models;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // server codes.
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

//---------------------------------------------------------
// Http server function
//---------------------------------------------------------
#[get("/")]
async fn index() -> AWResult<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).body("Success."))
}
//---------------------------------------------------------

//---------------------------------------------------------
// DB function
//---------------------------------------------------------

//prepare DBPool

lazy_static! {
    static ref DBPOOL: DbPool = {
        let mut loop_counter: u8 = 0;
        loop {
            if loop_counter == 255 {
                panic!("Can't build pool.");
            } else {
                loop_counter+=1;
            }

            dotenv().ok();

            let connspec = match std::env::var("DATABASE_URL") {
                Ok(url) => url, 
                Err(e) => {println!("{:?}", e); continue},
            };

            let manager = ConnectionManager::<PgConnection>::new(connspec);
            let pool = r2d2::Pool::builder()
                .connection_timeout(std::time::Duration::from_millis(500))
                .build(manager);
            match pool {
                Ok(p) => break p,
                Err(e) => println!("{:?}", e),
            }
        }
    };
}

use diesel::r2d2::{PooledConnection};
pub fn establish_connection() -> 
        PooledConnection<ConnectionManager<PgConnection>> {
    DBPOOL.clone().get().unwrap()
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
    let connection = establish_connection();
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