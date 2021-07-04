#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::env;

use anyhow::Result;
use infrastructure::rocket::rocket_init;

mod domain;
mod infrastructure;
mod usecase;

fn log_init() -> () {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn main() -> Result<()> {
    log_init();
    rocket_init();
    Ok(())
}
