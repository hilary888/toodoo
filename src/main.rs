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
use chrono:: Utc;

use models::{Todo, NewTodo, TodoData};
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

#[post("/", format = "json", data = "<user_input>")]
fn create_todo(user_input: Json<TodoData>) -> Json<Todo> {
    let todo = user_input.into_inner();
    let connection = establish_connection();

    let new_todo = NewTodo {
        title: Some(todo.title),
        body: Some(todo.body),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    let result = diesel::insert_into(todo::table)
        .values(&new_todo)
        .get_result(&connection)
        .expect("Error saving new todo list");

    Json(result)
}

#[get("/<id>")]
fn get_todo(id: i32) -> Json<Todo> {
    let connection = establish_connection();

    let result = todo::table
        .find(id)
        .first::<Todo>(&connection)
        .expect("Error loading user");

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
    rocket::build().mount("/", 
        routes![get_todos, create_todo, get_todo])
}