use anyhow::Result;
use rocket::Response;
use rocket::{http::Status, response::status::NotFound};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

use crate::domain::user::UserHasCharacter;
use crate::usecase::user_usecase;
use crate::{
    domain::user::token::Token,
    infrastructure::repository::user_repository_impl::UserRepositoryImpl,
};

#[post("/create", data = "<user_name>")]
pub fn user_create(
    user_name: Json<UserName>,
    user_repository: UserRepositoryImpl,
) -> Result<JsonValue> {
    let token = user_usecase::create_user(&user_name.name, &user_repository)?;
    Ok(json!({
        "token" : token.to_string(),
    }))
}

#[get("/get")]
pub fn user_get(
    token: Token,
    user_repository: UserRepositoryImpl,
) -> Result<Json<UserName>, NotFound<String>> {
    match user_usecase::find_by_token(&token, &user_repository) {
        Ok(user) => Ok(Json(UserName { name: user.name })),
        Err(err) => {
            error!("{}: {}", err, token.to_string());
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[put("/update", data = "<user_name>", format = "json")]
pub fn user_update(
    user_name: Json<UserName>,
    token: Token,
    user_repository: UserRepositoryImpl,
) -> Result<Status, NotFound<String>> {
    match user_usecase::update_name(&user_name.name, &token, &user_repository) {
        Ok(_) => Ok(Status::Ok),
        Err(err) => {
            error!("{}", err);
            Err(NotFound(format!("Token not found")))
        }
    }
}

#[get("/list")]
pub fn character_list_get(
    token: Token,
    user_repository: UserRepositoryImpl,
) -> Result<Json<UserHasCharacters>, Status> {
    let result = user_usecase::get_character_list(&token, &user_repository);

    match result {
        Ok(value) => Ok(Json(UserHasCharacters { characters: value })),
        Err(err) => {
            error!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserName {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserHasCharacters {
    characters: Vec<UserHasCharacter>,
}
