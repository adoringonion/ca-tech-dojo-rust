#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::{models::NewUser, schema::user};
use anyhow::Result;
use diesel::{self, insert_into, MysqlConnection, RunQueryDsl};

pub fn create_user(new_user_name: NewUser, conn: &MysqlConnection) -> Result<()> {
    insert_into(user::table)
        .values(&new_user_name)
        .execute(conn)?;

    Ok(())
}
