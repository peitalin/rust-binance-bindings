
#![allow(unused_variables)]
extern crate chrono;
extern crate regex;
extern crate clap;
extern crate scraper;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate strum_macros;
extern crate strum;

#[macro_use]
extern crate env_logger;
extern crate reqwest;

extern crate ring;
extern crate data_encoding;

extern crate actix;
extern crate actix_web;
extern crate futures;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate redis;

// pub mod coinmarketcap;
pub mod schema;
pub mod models;
pub mod serde_parsers;
pub mod binance;
pub mod db_actions;


use diesel::prelude::*;
use diesel::pg::PgConnection;



pub fn establish_connection_pg() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
