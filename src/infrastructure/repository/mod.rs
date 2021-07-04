use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    MysqlConnection,
};

pub mod user_repository;

type PooledMysqlConn = PooledConnection<ConnectionManager<MysqlConnection>>;
