use crate::domain::{
    game_character::{GameCharacter, GameCharacterRepository},
    user::{token::Token, UserRepository},
};

use anyhow::Result;

use super::gacha::draw_multiple;

pub fn draw_and_register(
    times: i32,
    token: &Token,
    user_repository: &impl UserRepository,
    game_character_repository: &impl GameCharacterRepository,
) -> Result<Vec<GameCharacter>> {
    let gacha_result = draw_multiple(times, game_character_repository)?;
    let user = user_repository.find_by_token(token)?;
    user_repository.register_character(user.get_id(), &gacha_result)?;

    Ok(gacha_result)
}
