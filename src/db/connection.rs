use anyhow::{Context, Result};
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
use std::{env, ops::Deref};

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
