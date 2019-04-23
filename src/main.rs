#![feature(decl_macro, proc_macro_hygiene)]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate rocket;
extern crate futures;
#[macro_use]
extern crate mysql_async as my;
extern crate tokio;

mod database;
mod graphql;
mod rocket_config;

fn main() {
    rocket_config::run();
}