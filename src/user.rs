use serde::Deserialize;
use serde::Serialize;

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
}
