use rocket::Rocket;

use crate::infrastructure::controller::{gacha_controller, user_controller};

use super::db::db_init;

pub fn rocket_init() {
    let rocket = rocket::ignite().manage(db_init());
    router(rocket).launch();
}

fn router(rocket: Rocket) -> rocket::Rocket {
    rocket
        .mount(
            "/user",
            routes![
                user_controller::user_create,
                user_controller::user_get,
                user_controller::user_update
            ],
        )
        .mount("/gacha", routes![gacha_controller::gacha_draw])
        .mount("/character", routes![user_controller::character_list_get])
}
