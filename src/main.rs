#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use serde::Serialize;

mod token;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

#[post("/create", data = "<new_user>")]
fn user_create(new_user: Json<User>) -> JsonValue {
    json!({
        "token" : token::Token::new().get(),
    })
}

#[get("/get")]
fn user_get() -> Json<User> {
    Json(User::new("test"))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/user", routes![user_create, user_get])
}

fn main() {
    rocket().launch();
}
