use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    MysqlConnection,
};

pub mod game_character_repository_impl;
pub mod user_repository_impl;

type PooledMysqlConn = PooledConnection<ConnectionManager<MysqlConnection>>;
