use crate::{
    domain::{game_character::GameCharacter, user::token::Token},
    infrastructure::repository::{
        game_character_repository_impl::GameCharacterRepositoryImpl,
        user_repository_impl::UserRepositoryImpl,
    },
    usecase::user_game_character::draw_and_register,
};
use anyhow::Result;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[post("/draw", data = "<draw_times>", format = "json")]
pub fn gacha_draw(
    draw_times: Json<Times>,
    token: Token,
    user_repository: UserRepositoryImpl,
    game_character_repository: GameCharacterRepositoryImpl,
) -> Result<Json<GachaResult>> {
    let results = draw_and_register(
        draw_times.times,
        &token,
        &user_repository,
        &game_character_repository,
    )?;

    Ok(Json(GachaResult { results }))
}

#[derive(Serialize, Deserialize)]
pub struct Times {
    times: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GachaResult {
    results: Vec<GameCharacter>,
}
