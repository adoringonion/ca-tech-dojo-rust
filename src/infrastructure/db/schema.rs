table! {
    user (id) {
        id -> Integer,
        name -> Text,
        token -> Text,
    }
}

table! {
    user_has_character (id) {
        id -> Integer,
        user_id -> Integer,
        character_id -> Integer,
        quantity -> Integer,
    }
}

table! {
    game_character (id) {
        id -> Integer,
        name -> Text,
        rarity -> Integer,
    }
}

joinable!(game_character  -> user_has_character (id));
allow_tables_to_appear_in_same_query!(game_character, user_has_character);
