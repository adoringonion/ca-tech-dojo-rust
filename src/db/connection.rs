use anyhow::Result;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod db_connection {
    pub fn establish_connection() -> Result<MysqlConnection> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        MysqlConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
