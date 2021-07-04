use crate::domain::game_character::GameCharacter;
use crate::domain::user::token::Token;
use crate::infrastructure::repository::game_character_repository_impl::GameCharacterRepositoryImpl;
use crate::usecase::gacha::draw_multiple;
use anyhow::Result;

use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;

#[post("/draw", data = "<draw_times>", format = "json")]
pub fn gacha_draw(
    draw_times: Json<Times>,
    token: Token,
    game_character_repository: GameCharacterRepositoryImpl,
) -> Result<Json<GachaResult>> {
    let results = draw_multiple(draw_times.times, &game_character_repository)?;
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
