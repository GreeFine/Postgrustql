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

#[derive(juniper::GraphQLObject, Debug)]
struct Picture {
    binaire_href: String,
    determination_ns: String,
}

#[derive(juniper::GraphQLObject, Debug)]
struct Pictures {
    pub nodes: Vec<Picture>,
}

impl database::requestable_data for Picture {
    fn new() -> Self {
        Picture {
            binaire_href: String::new(),
            determination_ns: String::new(),
        }
    }
    fn create(row: &mut mysql::Row) -> Self {
        let mut _self = Self::new();
        _self.feed(row);
        _self
    }

    fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
        self.binaire_href = row.take(0).unwrap();
        self.determination_ns = row.take(1).unwrap();
        self
    }
}


impl database::requestable_data for Pictures {
    fn new() -> Self {
        Pictures { nodes: Vec::new() }
    }

    fn create(_: &mut mysql::Row) -> Self {
        panic!("Not implem")
    }

    fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
        self.nodes.push(Picture::create(row));
        self
    }
}

fn main() {
    let db = database::Database::new();
    let pics = db.request::<Pictures>(
        vec!["binaire_href", "determination_ns"],
        database::e_tables::pictures,
        Some(2),
    );
    println!("{:?}", pics);
    // rocket_config::run();
}