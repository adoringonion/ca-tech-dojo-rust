use super::schema::{token, user};

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "token"]
pub struct NewToken {
    pub value: String,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "user"]
pub struct User {
    pub name: String,
}
