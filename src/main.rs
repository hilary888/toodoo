#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
extern crate dotenv;

mod db;
mod models;
mod schema;

use chrono::Utc;
use db::establish_connection;
use diesel::prelude::*;
use models::{NewTodo, Todo, TodoData};
use rocket::serde::json::{json, Json, Value};
use schema::*;

#[get("/")]
fn get_todos() -> Json<Value> {
    use crate::schema::todo::dsl::*;
    let connection = establish_connection();
    let result = todo.load::<Todo>(&connection).expect("Error loading todo");

    Json(json!({ "data": result }))
}

#[post("/", format = "json", data = "<user_input>")]
fn create_todo(user_input: Json<TodoData>) -> Json<Value> {
    let todo = user_input.into_inner();
    let connection = establish_connection();

    let new_todo = NewTodo {
        title: Some(todo.title),
        body: Some(todo.body),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    let result: Todo = diesel::insert_into(todo::table)
        .values(&new_todo)
        .get_result(&connection)
        .expect("Error saving new todo list");

    Json(json!({ "data": result }))
}

#[get("/<id>")]
fn get_todo(id: i32) -> Json<Value> {
    use crate::schema::todo::dsl::*;
    let connection = establish_connection();

    let result = todo
        .find(id)
        .first::<Todo>(&connection)
        .expect("Error loading user");

    Json(json!({ "data": result }))
}

#[delete("/<id>")]
fn delete_todo(id: i32) -> Json<Value> {
    use crate::schema::todo::dsl::*;
    let connection = establish_connection();

    let result = diesel::delete(todo.find(id)).execute(&connection).is_ok();
    Json(json!({
        "success": result,
    }))
}

#[put("/<id>", format = "json", data = "<data>")]
fn update_todo(id: i32, data: Json<TodoData>) -> Json<Value> {
    let todo = data.into_inner();
    let connection = establish_connection();

    let updated_todo = NewTodo {
        title: Some(todo.title),
        body: Some(todo.body),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    let result: Todo = diesel::update(todo::table.find(id))
        .set(&updated_todo)
        .get_result(&connection)
        .expect("Error updating todo");

    Json(json!({ "data": result }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![get_todos, create_todo, get_todo, update_todo, delete_todo],
    )
}
