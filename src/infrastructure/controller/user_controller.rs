use anyhow::Result;
use rocket::{http::Status, response::status::NotFound};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::{
    domain::user::{token::Token, User},
    infrastructure::repository::user_repository::UserRepository,
};

#[post("/create", data = "<new_user>")]
pub fn user_create(new_user: Json<User>, user_repository: UserRepository) -> Result<JsonValue> {
    let token = Token::generate();
    user_repository.create(new_user.0, &token)?;
    Ok(json!({
        "token" : token.to_string(),
    }))
}

#[get("/get")]
pub fn user_get(
    token: Token,
    user_repository: UserRepository,
) -> Result<Json<User>, NotFound<String>> {
    match user_repository.find_by_token(&token) {
        Ok(user) => Ok(Json(User::from_model(user))),
        Err(err) => {
            error!("{}: {}", err, token.to_string());
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[put("/update", data = "<user>", format = "json")]
pub fn user_update(
    user: Json<User>,
    token: Token,
    user_repository: UserRepository,
) -> Result<Status, NotFound<String>> {
    match user_repository.update(user.0.name, &token) {
        Ok(_) => Ok(Status::Ok),
        Err(err) => {
            error!("{}", err);
            Err(NotFound(format!("Token not found")))
        }
    }
}
