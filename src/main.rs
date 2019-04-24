#![feature(decl_macro, proc_macro_hygiene)]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate mysql;
extern crate tokio;
#[macro_use]
extern crate strum_macros;

pub mod database;
mod graphql;
mod rocket_config;


fn main() {
    rocket_config::run();
}