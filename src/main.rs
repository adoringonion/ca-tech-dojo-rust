#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use crate::user::User;
use db::connection;
use rocket::http::Status;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use token::Token;

mod db;
mod repository;
mod token;
mod user;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>, db: connection::DbConn) -> JsonValue {
    let token = Token::generate();
    repository::create_user(new_user.to_model(), &db);
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
    let connection = db::connection::establish();
    match connection {
        Ok(connection) => rocket::ignite()
            .manage(connection)
            .mount("/user", routes![user_create, user_get, user_update]),
        Err(_) => todo!(),
    }
}

fn main() {
    rocket().launch();
}
