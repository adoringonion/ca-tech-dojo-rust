use crate::domain::game_character::GameCharacter;
use crate::domain::game_character::Rarity;

use super::schema::game_character;
use super::schema::user;

#[derive(Insertable, Debug)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub token: String,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: String,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "game_character"]
pub struct GameCharacterModel {
    pub id: i32,
    pub name: String,
    pub rarity: i32,
}

impl GameCharacterModel {

    pub fn number_to_rarity(&self) -> Rarity {
        match self.rarity {
            0 => Rarity::SuperUltraRare,
            1 => Rarity::UltraRare,
            2 => Rarity::Rare,
            3 => Rarity::Common,
            _ => panic!("Unexpected number"),
        }
    }
}

impl Into<GameCharacter> for GameCharacterModel {
    fn into(self) -> GameCharacter {
        GameCharacter::new(self.id, self.name.clone(), self.number_to_rarity())
    }

}