pub mod token;

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use self::token::Token;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    pub name: String,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        User { id, name }
    }
}

pub trait UserRepository {
    fn create(&self, name: &String, token: &Token) -> Result<()>;
    fn find_by_token(&self, token: &Token) -> Result<User>;
    fn update(&self, new_name: &String, token: &Token) -> Result<()>;
}
