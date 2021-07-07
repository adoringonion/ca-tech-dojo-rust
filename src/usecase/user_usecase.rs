use crate::domain::user::{token::Token, User, UserHasCharacter, UserRepository};
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

pub fn update_name(
    name: &String,
    token: &Token,
    user_repository: &impl UserRepository,
) -> Result<()> {
    user_repository.update_name(name, token)?;
    Ok(())
}

pub fn get_character_list(
    token: &Token,
    user_repository: &impl UserRepository,
) -> Result<Vec<UserHasCharacter>> {
    let user = user_repository.find_by_token(token)?;
    let result = user_repository.get_character_list(user.get_id())?;

    Ok(result)
}
