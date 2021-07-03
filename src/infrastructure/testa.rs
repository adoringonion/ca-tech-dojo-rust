use anyhow::{anyhow, Result};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use crate::domain::user::token::Token;

use super::db::MysqlPool;

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

fn is_valid(key: &str) -> Result<Token> {
    if key == "" {
        return Err(anyhow!("null"));
    }

    return Token::new(key);
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("x-token");
        match token {
            Some(token) => match is_valid(token) {
                Ok(token) => Outcome::Success(token),
                Err(_) => Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid)),
            },
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}
