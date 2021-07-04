#![allow(proc_macro_derive_resolution_fallback)]

use crate::{
    domain::user::{token::Token, User, UserRepository},
    infrastructure::db::{
        models::{NewUser, User as UserModel},
        schema::user::{self, dsl::*},
        MysqlPool,
    },
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

pub struct UserRepositoryImpl {
    db_conn: PooledMysqlConn,
}

impl UserRepository for UserRepositoryImpl {
    fn create(&self, new_name: &String, new_token: &Token) -> Result<()> {
        let new_user = NewUser {
            name: new_name.clone(),
            token: new_token.to_string(),
        };

        insert_into(user::table)
            .values(new_user)
            .execute(&self.db_conn)?;
        Ok(())
    }

    fn find_by_token(&self, input_token: &Token) -> Result<User> {
        let result = user
            .filter(token.eq(input_token.to_string()))
            .first::<UserModel>(&self.db_conn)?;

        Ok(User::new(result.id, result.name))
    }

    fn update(&self, new_name: &String, input_token: &Token) -> Result<()> {
        diesel_update(user.filter(token.eq(input_token.to_string())))
            .set(name.eq(new_name))
            .execute(&self.db_conn)?;
        Ok(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRepositoryImpl {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRepositoryImpl, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(UserRepositoryImpl { db_conn: conn }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
