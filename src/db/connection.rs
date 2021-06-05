use anyhow::{Context, Result};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish() -> Result<MysqlConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").with_context(|| format!("DATABASE_URL must be set"))?;
    Ok(MysqlConnection::establish(&database_url)
        .with_context(|| format!("Error connectiong to {}", database_url))?)
}