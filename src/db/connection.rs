use anyhow::{Context, Result};
use core::time;

use dotenv::dotenv;

use sqlx::mysql::MySqlPoolOptions;
use std::{env, thread};

fn database_url() -> Result<String> {
    dotenv().ok();
    Ok(env::var("DATABASE_URL").with_context(|| format!("DATABASE_URL must be set"))?)
}

async fn init() -> Result<sqlx::MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url()?)
        .await?;

    Ok(pool)
}

pub async fn create_db_pool() -> sqlx::MySqlPool {
    let db_connection = loop {
        let mut counter = 0;
        match init().await {
            Ok(connection) => {
                info!("Successed DB Connection");
                break connection;
            }
            Err(err) => {
                thread::sleep(time::Duration::from_secs(3));
                if counter < 5 {
                    error!("{}", err);
                    counter = counter + 1;
                } else {
                    panic!("Cant connect to DB")
                }
            }
        }
    };

    return db_connection;
}
