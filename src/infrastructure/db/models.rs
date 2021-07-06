use num::FromPrimitive;

use crate::domain::game_character::GameCharacter;
use crate::domain::game_character::Rarity;

use super::schema::game_character;
use super::schema::user;
use super::schema::user_has_character;

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

impl Into<GameCharacter> for GameCharacterModel {
    fn into(self) -> GameCharacter {
        GameCharacter::new(
            self.id,
            self.name.clone(),
            Rarity::from_i32(self.rarity).unwrap(),
        )
    }
}
