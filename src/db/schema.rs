table! {
    token (id) {
        id -> Integer,
        value -> Varchar,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Varchar,
        token_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    token,
    user,
);
