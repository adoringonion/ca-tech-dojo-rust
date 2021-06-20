#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use core::time;
use std::{env, thread};

use crate::user::User;
use anyhow::Result;
use db::connection;
use repository::create_user;
use rocket::http::Status;
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

async fn rocket() -> sqlx::MySqlPool {
    let db_connection = loop {
        let mut counter = 0;
        match db::connection::init().await {
            Ok(connection) => {
                info!("Successed DB Connection");
                break connection;
            }
            Err(err) => {
                thread::sleep(time::Duration::from_secs(3));
                if counter < 5 {
                    error!("{}", err);
                    counter = counter + 1;
                } else {
                    panic!("Cant connect to DB")
                }
            }
        }
    };

    return db_connection;
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    rocket::ignite()
        .manage(rocket().await)
        .mount("/user", routes![user_create, user_get, user_update])
        .launch();
    Ok(())
}
