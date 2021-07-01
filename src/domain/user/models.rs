use super::schema::user;

#[derive(Insertable, Debug)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub token: String,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub token: String,
}
