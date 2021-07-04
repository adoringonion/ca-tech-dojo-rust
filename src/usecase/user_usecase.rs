use crate::domain::user::{token::Token, User, UserRepository};
use anyhow::Result;

pub fn create_user(name: &String, user_repository: &impl UserRepository) -> Result<Token> {
    let new_token = Token::generate();
    match user_repository.create(name, &new_token) {
        Ok(_) => Ok(new_token),
        Err(err) => Err(err),
    }
}

pub fn find_by_token(token: &Token, user_repository: &impl UserRepository) -> Result<User> {
    Ok(user_repository.find_by_token(token)?)
}

pub fn update(name: &String, token: &Token, user_repository: &impl UserRepository) -> Result<()> {
    user_repository.update(name, token)?;
    Ok(())
}
