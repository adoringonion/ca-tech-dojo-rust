#![allow(proc_macro_derive_resolution_fallback)]

use diesel::{self, RunQueryDsl, Connection};
use crate::db::{connection::DbConn, models::NewUser, schema::user};

pub fn create_user(new_user_name: NewUser, conn: &DbConn) {
    diesel::insert_into(user::table).values(&new_user_name).execute(conn);
}