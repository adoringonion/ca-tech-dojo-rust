use anyhow::{Context, Result};
use core::time;
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};
use sqlx::mysql::MySqlPoolOptions;
use std::{env, ops::Deref, thread};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish() -> Result<MysqlPool> {
    info!("Try to connect DB...");
    info!("DB_URL {}", database_url()?);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url()?);
    Ok(Pool::new(manager).with_context(|| "Failed to create pool")?)
}

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

pub struct DbConn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
