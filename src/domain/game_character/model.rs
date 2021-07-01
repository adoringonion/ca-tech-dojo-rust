use super::schema::game_character;

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "game_character"]
pub struct GameCharacter {
    pub id: i32,
    pub name: String,
    pub rarity: i32,
}
