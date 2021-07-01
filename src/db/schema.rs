table! {
    user (id) {
        id -> Integer,
        name -> Text,
        token -> Text,
    }
}

table! {
    game_character (id) {
        id -> Integer,
        name -> Text,
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
