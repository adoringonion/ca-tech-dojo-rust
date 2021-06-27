#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::env;

use crate::db::connection::db_init;
use crate::user::User;
use anyhow::Result;
use db::connection;
use repository::{create_user, find_by_token, update_user};
use rocket::http::Status;

use rocket::response::status::NotFound;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use token::Token;

mod db;
mod repository;
mod token;
mod user;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>, db: connection::DbConn) -> Result<JsonValue> {
    let token = Token::generate();
    create_user(new_user.0, &token, &db)?;
    Ok(json!({
        "token" : token.to_string(),
    }))
}

#[get("/get")]
fn user_get(token: Token, db: connection::DbConn) -> Result<Json<User>, NotFound<String>> {
    match find_by_token(&token, &db) {
        Ok(user) => Ok(Json(User::from_model(user))),
        Err(err) => {
            error!("{}: {}", err, token.to_string());
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[put("/update", data = "<user>", format = "json")]
fn user_update(
    user: Json<User>,
    token: Token,
    db: connection::DbConn,
) -> Result<Status, NotFound<String>> {
    match update_user(user.0.name, &token, &db) {
        Ok(_) => Ok(Status::Ok),
        Err(err) => {
            error!("{}", err);
            Err(NotFound(format!("Token not found")))
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db_init())
        .mount("/user", routes![user_create, user_get, user_update])
}

fn log_init() -> () {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn main() -> Result<()> {
    log_init();
    rocket().launch();
    Ok(())
}
