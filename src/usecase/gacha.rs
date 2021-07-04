use anyhow::Result;
use rand::{prelude::SliceRandom, Rng};

use crate::domain::game_character::{GameCharacter, GameCharacterRepository, Rarity};

fn draw(game_character_repository: &impl GameCharacterRepository) -> Result<GameCharacter> {
    let rarity = choose_rarity();
    let characters = game_character_repository.find_by_rarity(rarity)?;
    Ok(choose_character(characters))
}

pub fn draw_multiple(
    times: i32,
    game_character_repository: &impl GameCharacterRepository,
) -> Result<Vec<GameCharacter>> {
    let mut gacha_result: Vec<GameCharacter> = Vec::new();

    for _ in 0..times {
        gacha_result.push(draw(game_character_repository)?)
    }

    Ok(gacha_result)
}

fn choose_rarity() -> Rarity {
    match rand::thread_rng().gen_range(1..100) {
        1..=2 => Rarity::SuperUltraRare,
        3..=12 => Rarity::UltraRare,
        13..=32 => Rarity::Rare,
        _ => Rarity::Common,
    }
}

fn choose_character(characters: Vec<GameCharacter>) -> GameCharacter {
    match characters.choose(&mut rand::thread_rng()) {
        Some(character) => character.clone(),
        None => panic!(),
    }
}
