use rand::Rng;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request,
};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const TOKEN_LENGTH: usize = 20;

pub struct Token {
    value: String,
}

#[derive(Debug)]
pub enum ApiTokenError {
    Missing,
    Invalid,
}

fn is_valid(key: &str) -> bool {
    if key == "" {
        return false;
    }

    return true;
}

impl Token {
    fn new(token: &str) -> Self {
        Token {
            value: token.to_string(),
        }
    }

    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        let value: String = (0..TOKEN_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        return Token { value };
    }

    pub fn to_string(self) -> String {
        self.value
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ApiTokenError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("x-token");
        match token {
            Some(token) => {
                if is_valid(token) {
                    Outcome::Success(Token::new(token))
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
                }
            }

            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        }
    }
}
