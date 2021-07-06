use crate::domain::{
    game_character::{GameCharacter, GameCharacterRepository},
    user::UserRepository,
};

use anyhow::Result;

use super::gacha::draw_multiple;

pub fn draw_and_resister(
    times: i32,
    user_repository: &impl UserRepository,
    game_character_repository: &impl GameCharacterRepository,
) -> Result<Vec<GameCharacter>> {
    let gacha_result = draw_multiple(times, game_character_repository)?;

    Ok(gacha_result)
}
