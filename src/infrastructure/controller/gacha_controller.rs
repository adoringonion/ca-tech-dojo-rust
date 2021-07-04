use crate::domain::user::token::Token;
use anyhow::Result;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde::Serialize;

#[post("/draw", data = "<draw_times>", format = "json")]
pub fn gacha_draw(draw_times: Json<Times>, token: Token) -> Result<JsonValue> {
    Ok(json!({ "times" : 0}))
}

#[derive(Serialize, Deserialize)]
pub struct Times {
    value: i32,
}
