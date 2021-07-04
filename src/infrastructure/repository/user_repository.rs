#![allow(proc_macro_derive_resolution_fallback)]

use crate::{
    domain::user::{
        models::User as UserModel,
        schema::user::{self, dsl::*},
        token::Token,
        User,
    },
    infrastructure::db::MysqlPool,
};

use anyhow::Result;
use diesel::{
    query_builder::functions::{insert_into, update as diesel_update},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use super::PooledMysqlConn;

pub struct UserRepository {
    db_conn: PooledMysqlConn,
}

impl UserRepository {
    fn new(conn: PooledMysqlConn) -> Self {
        Self { db_conn: conn }
    }

    pub fn create(&self, new_user: User, new_token: &Token) -> Result<()> {
        insert_into(user::table)
            .values(new_user.to_model(new_token.to_string()))
            .execute(&self.db_conn)?;
        Ok(())
    }

    pub fn find_by_token(&self, input_token: &Token) -> Result<UserModel> {
        Ok(user
            .filter(token.eq(input_token.to_string()))
            .first::<UserModel>(&self.db_conn)?)
    }

    pub fn update(&self, new_name: String, input_token: &Token) -> Result<()> {
        diesel_update(user.filter(token.eq(input_token.to_string())))
            .set(name.eq(new_name))
            .execute(&self.db_conn)?;
        Ok(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRepository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRepository, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(UserRepository::new(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
