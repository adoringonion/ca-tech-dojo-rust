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
use std::{env, ops::Deref, thread};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

fn establish() -> Result<MysqlPool> {
    info!("Try to connect DB...");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url()?);
    Ok(Pool::new(manager).with_context(|| "Failed to create pool")?)
}

fn database_url() -> Result<String> {
    if let Err(_) = env::var("DATABASE_URL") {
        dotenv().ok();
    }

    Ok(env::var("DATABASE_URL").with_context(|| format!("DATABASE_URL must be set"))?)
}

pub fn db_init() -> MysqlPool {
    let db_connection = loop {
        let mut counter = 0;
        match establish() {
            Ok(connection) => {
                info!("Successed DB Connection");
                break connection;
            }
            Err(err) => {
                thread::sleep(time::Duration::from_secs(3));
                if counter < 5 {
                    error!("{}", err);
                } else {
                    panic!("Cant connect to DB")
                }
            }
        }
        counter += 1;
        info!("Tried {} times", counter);
    };

    db_connection
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
