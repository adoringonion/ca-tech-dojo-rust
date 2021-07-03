use anyhow::Result;
use uuid::Uuid;

pub struct Token {
    value: Uuid,
}

impl Token {
    pub fn new(input: &str) -> Result<Token> {
        let token = Uuid::parse_str(input)?;
        Ok(Token { value: token })
    }

    pub fn generate() -> Self {
        return Self {
            value: Uuid::new_v4(),
        };
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}
