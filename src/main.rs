#[macro_use] extern crate rocket;
use rocket::serde::json::{json, Value, Json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Todo {
    id: usize,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
}
#[get("/<id>")]
fn get_todo(id: usize) -> Json<Todo> {
    Json(Todo{ 
        id: id, 
        title: String::from("sfads"), 
        body: String::from("sfads"), 
        created_at: String::from("sfads"), 
        updated_at: String::from("sfads"),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_todo])
}