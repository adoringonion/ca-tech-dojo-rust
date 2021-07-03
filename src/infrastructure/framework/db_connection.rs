use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    MysqlConnection,
};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use crate::infrastructure::db::MysqlPool;

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
