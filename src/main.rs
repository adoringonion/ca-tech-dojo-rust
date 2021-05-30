#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::Deserialize;

#[derive(Deserialize)]
struct NewUser {
    name: String,
}

#[post("/create", data = "<new_user>")]
fn user_create(new_user: Json<NewUser>) -> JsonValue {
    json!({
        "token" : "test",
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/user", routes![user_create])
}

fn main() {
    rocket().launch();
}
