pub mod models;
pub mod schema;
pub mod token;

use serde::Deserialize;
use serde::Serialize;

use self::models::{NewUser, User as UserModel};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }

    pub fn to_model(&self, token: String) -> NewUser {
        NewUser {
            name: self.name.clone(),
            token,
        }
    }
    pub fn from_model(model: UserModel) -> Self {
        User { name: model.name }
    }
}
