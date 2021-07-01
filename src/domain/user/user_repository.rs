#![allow(proc_macro_derive_resolution_fallback)]

use super::{
    models::User as UserModel,
    schema::user::{self, dsl::*},
    token::Token,
    User,
};
use anyhow::Result;
use diesel::{
    mysql::MysqlConnection,
    query_builder::functions::{insert_into, update as diesel_update},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};

pub fn create(new_user: User, new_token: &Token, conn: &MysqlConnection) -> Result<()> {
    insert_into(user::table)
        .values(new_user.to_model(new_token.to_string()))
        .execute(conn)?;
    Ok(())
}

pub fn find_by_token(input_token: &Token, conn: &MysqlConnection) -> Result<UserModel> {
    Ok(user
        .filter(token.eq(input_token.to_string()))
        .first::<UserModel>(conn)?)
}

pub fn update(new_name: String, input_token: &Token, conn: &MysqlConnection) -> Result<()> {
    diesel_update(user.filter(token.eq(input_token.to_string())))
        .set(name.eq(new_name))
        .execute(conn)?;
    Ok(())
}
