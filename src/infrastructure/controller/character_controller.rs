use crate::domain::user::token::Token;
use anyhow::Result;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[get("/list")]
pub fn character_list(token: Token) -> Result<JsonValue> {
    Ok(json!({
        "token" : token.to_string(),
    }))
}
