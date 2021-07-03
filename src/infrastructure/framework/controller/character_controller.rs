use crate::domain::user::token::Token;
use crate::infrastructure::framework::db_connection::DbConn;
use anyhow::Result;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[get("/list")]
pub fn character_list(token: Token, db: DbConn) -> Result<JsonValue> {
    Ok(json!({
        "token" : token.to_string(),
    }))
}
