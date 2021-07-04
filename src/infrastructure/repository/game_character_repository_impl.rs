use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

use anyhow::Result;
use diesel::{
    query_builder::functions::{insert_into, update as diesel_update},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};

use crate::{
    domain::game_character::{GameCharacter, GameCharacterRepository, Rarity},
    infrastructure::db::{
        models::GameCharacterModel,
        schema::game_character::{self, dsl::*},
        MysqlPool,
    },
};

use super::PooledMysqlConn;

pub struct GameCharacterRepositoryImpl {
    db_conn: PooledMysqlConn,
}

impl GameCharacterRepository for GameCharacterRepositoryImpl {
    fn find_by_rarity(&self, other_rarity: Rarity) -> Result<Vec<GameCharacter>> {
        let result = game_character
            .filter(rarity.eq(other_rarity as i32))
            .load::<GameCharacterModel>(&self.db_conn)?;

        let aaa: Vec<GameCharacter> = result.into_iter().map(|x| x.into()).collect();
        Ok(aaa)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for GameCharacterRepositoryImpl {
    type Error = ();

    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<GameCharacterRepositoryImpl, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(GameCharacterRepositoryImpl { db_conn: conn }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
