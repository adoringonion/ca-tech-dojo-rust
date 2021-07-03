pub mod model;
pub mod schema;

enum Rarity {
    SuperUltraRare,
    UltraRare,
    Rare,
    Common,
}

pub struct GameCharacter {
    id: i32,
    name: String,
    rarity: Rarity,
}

pub fn draw() {}
