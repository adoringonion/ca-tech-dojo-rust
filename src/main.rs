#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use crate::user::User;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use token::Token;
use crate::db;

mod token;
mod user;
moddb_connection;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>) -> JsonValue {
    let token = Token::generate();
    json!({
        "token" : token.to_string(),
    })
}

#[get("/get")]
fn user_get(token: Token) -> Json<User> {
    if token.to_string() != "" {
        return Json(User::new("chinchin"));
    }
    Json(User::new("test"))
}

#[put("/update", data = "<user>", format = "json")]
fn user_update(user: Json<User>, token: Token) -> Status {
    Status::Ok
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/user", routes![user_create, user_get, user_update])
}

fn main() {
    rocket().launch();
}
