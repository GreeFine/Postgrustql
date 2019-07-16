#![feature(decl_macro, proc_macro_hygiene, trace_macros)]
#[macro_use]
extern crate rocket;
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate tokio;
extern crate uuid;

mod database;
mod graphql;
mod rocket_config;

fn main() {
    rocket_config::run();
}
