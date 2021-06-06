#![allow(proc_macro_derive_resolution_fallback)]

use crate::{db::schema::{self, token, user}, token::{ Token}, user::User};
use anyhow::Result;
use diesel::{self, MysqlConnection, RunQueryDsl, insert_into, select};

pub fn create_user(new_user: User, new_token: &Token, conn: &MysqlConnection) -> Result<()> {
    insert_into(token::table)
        .values(new_token.to_model())
        .execute(conn)?;
    let id = select(expression)
    insert_into(user::table)
        .values(new_user.to_model(token_id as i32))
        .execute(conn)?;

    Ok(())
}
