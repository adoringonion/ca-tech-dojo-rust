pub mod model;
pub mod schema;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Rarity {
    SuperUltraRare,
    UltraRare,
    Rare,
    Common,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameCharacter {
    id: i32,
    name: String,
    rarity: Rarity,
}

impl GameCharacter {
    pub fn new(id: i32, name: String, rarity: Rarity) -> Self {
        Self { id, name, rarity }
    }
}
pub trait GameCharacterRepository {
    fn find_by_rarity(&self, rarity: Rarity) -> Result<Vec<GameCharacter>>;
}
