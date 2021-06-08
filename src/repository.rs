#![allow(proc_macro_derive_resolution_fallback)]

use crate::{db::schema::user, token::Token, user::User};
use anyhow::Result;
use diesel::{self, insert_into, MysqlConnection, RunQueryDsl};

pub fn create_user(new_user: User, new_token: &Token, conn: &MysqlConnection) -> Result<()> {
    insert_into(user::table)
        .values(new_user.to_model(new_token.to_string()))
        .execute(conn)?;

    Ok(())
}
