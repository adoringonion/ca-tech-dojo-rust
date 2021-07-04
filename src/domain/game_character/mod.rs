use anyhow::Result;
use num::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Rarity {
    SuperUltraRare,
    UltraRare,
    Rare,
    Common,
}

impl FromPrimitive for Rarity {
    fn from_i32(n: i32) -> Option<Self> {
        match n {
            0 => Some(Rarity::SuperUltraRare),
            1 => Some(Rarity::UltraRare),
            2 => Some(Rarity::Rare),
            3 => Some(Rarity::Common),
            _ => None,
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Rarity::SuperUltraRare),
            1 => Some(Rarity::UltraRare),
            2 => Some(Rarity::Rare),
            3 => Some(Rarity::Common),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Rarity::SuperUltraRare),
            1 => Some(Rarity::UltraRare),
            2 => Some(Rarity::Rare),
            3 => Some(Rarity::Common),
            _ => None,
        }
    }
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
