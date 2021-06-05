use serde::Deserialize;
use serde::Serialize;

use crate::db::models::NewUser;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }

    pub fn to_model(&self) -> NewUser {
        NewUser {
            name: self.name.clone(),
        }
    }
}
