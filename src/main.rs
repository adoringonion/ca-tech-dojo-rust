#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::env;

use anyhow::Result;
use domain::user::{
    token::Token,
    user_repository::{create, find_by_token, update},
    User,
};
use rocket::{http::Status, response::status::NotFound};

use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::db::{db_init, DbConn};

mod db;
mod domain;

#[post("/create", data = "<new_user>", format = "json")]
fn user_create(new_user: Json<User>, db: DbConn) -> Result<JsonValue> {
    let token = Token::generate();
    create(new_user.0, &token, &db)?;
    Ok(json!({
        "token" : token.to_string(),
    }))
}

#[get("/get")]
fn user_get(token: Token, db: DbConn) -> Result<Json<User>, NotFound<String>> {
    match find_by_token(&token, &db) {
        Ok(user) => Ok(Json(User::from_model(user))),
        Err(err) => {
            error!("{}: {}", err, token.to_string());
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[put("/update", data = "<user>", format = "json")]
fn user_update(user: Json<User>, token: Token, db: DbConn) -> Result<Status, NotFound<String>> {
    match update(user.0.name, &token, &db) {
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
