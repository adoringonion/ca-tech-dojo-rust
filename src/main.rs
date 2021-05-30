#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use crate::user::User;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod token;
mod user;

#[post("/create", data = "<new_user>")]
fn user_create(new_user: Json<User>) -> JsonValue {
    let token = token::Token::new();
    json!({
        "token" : token.to_string(),
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
