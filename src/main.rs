#![feature(decl_macro, proc_macro_hygiene, trace_macros)]
#[macro_use]
extern crate rocket;
extern crate juniper;
extern crate juniper_rocket;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate tokio;

mod database;
mod graphql;
mod rocket_config;

fn main() {
    rocket_config::run();
}