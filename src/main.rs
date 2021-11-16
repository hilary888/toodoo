#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
extern crate dotenv;

pub mod models;
pub mod schema;

use rocket::serde::json::{json, Value, Json};
// use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;




use models::Todo;
use diesel::prelude::*;
use schema::*;

#[get("/")]
fn get_todos() -> Json<Vec<Todo>>{
    let connection = establish_connection();
    let result = todo::table
        .load::<Todo>(&connection)
        .expect("Error loading todo");
        
    Json(result)
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_todos])
}