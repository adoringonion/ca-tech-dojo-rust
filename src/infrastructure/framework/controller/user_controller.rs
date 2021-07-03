use anyhow::Result;
use rocket::{http::Status, response::status::NotFound};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::domain::user::{
    token::Token,
    user_repository::{create, find_by_token, update},
    User,
};
use crate::infrastructure::framework::db_connection::DbConn;

#[post("/create", data = "<new_user>")]
pub fn user_create(new_user: Json<User>, db: DbConn) -> Result<JsonValue> {
    let token = Token::generate();
    create(new_user.0, &token, &db.0)?;
    Ok(json!({
        "token" : token.to_string(),
    }))
}

#[get("/get")]
pub fn user_get(token: Token, db: DbConn) -> Result<Json<User>, NotFound<String>> {
    match find_by_token(&token, &db.0) {
        Ok(user) => Ok(Json(User::from_model(user))),
        Err(err) => {
            error!("{}: {}", err, token.to_string());
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[put("/update", data = "<user>", format = "json")]
pub fn user_update(user: Json<User>, token: Token, db: DbConn) -> Result<Status, NotFound<String>> {
    match update(user.0.name, &token, &db.0) {
        Ok(_) => Ok(Status::Ok),
        Err(err) => {
            error!("{}", err);
            Err(NotFound(format!("Token not found")))
        }
    }
}
