#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use crate::user::User;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

mod token;
mod user;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>) -> JsonValue {
    let token = token::Token::new();
    json!({
        "token" : token.to_string(),
    })
}

#[get("/get", format = "json")]
fn user_get() -> Json<User> {
    Json(User::new("test"))
}

#[put("/update", data = "<user>", format = "json")]
fn user_update(user: Json<User>) -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/user", routes![user_create, user_get, user_update])
}

fn main() {
    rocket().launch();
}
