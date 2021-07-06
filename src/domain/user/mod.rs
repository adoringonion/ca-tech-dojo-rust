pub mod token;

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use self::token::Token;

use super::game_character::GameCharacter;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        User { id, name }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub trait UserRepository {
    fn create(&self, name: &String, token: &Token) -> Result<()>;
    fn find_by_token(&self, token: &Token) -> Result<User>;
    fn update_name(&self, new_name: &String, token: &Token) -> Result<()>;
    fn register_character(&self, user_id: i32, gacha_result: &Vec<GameCharacter>) -> Result<()>;
}
