#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

use std::env;

use crate::db::connection::create_db_pool;
use crate::user::User;
use anyhow::Result;
use db::connection;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use sqlx::MySqlPool;
use token::Token;

mod db;
mod repository;
mod token;
mod user;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>, db: State<MySqlPool>) -> Result<JsonValue> {
    let token = Token::generate();
    Ok(json!({
        "token" : token.to_string(),
    }))
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

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    rocket::ignite()
        .manage(create_db_pool().await)
        .mount("/user", routes![user_create, user_get, user_update])
        .launch();
    Ok(())
}
