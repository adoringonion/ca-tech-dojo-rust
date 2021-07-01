pub mod model;
pub mod schema;

enum Rarity {
    SuperUltraRare,
    UltraRare,
    Rare,
    Common,
}

pub struct GameCharacter {
    name: String,
    rarity: Rarity,
}
