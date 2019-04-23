use super::graphql::queries::{FlowerDesc, Flowers};
use futures::Future;
use mysql::prelude::*;
use mysql::OptsBuilder;

pub struct Database {
  pool: mysql::Pool,
}

impl Database {
  pub fn new() -> Database {
    let mut database = OptsBuilder::new();
    database
      .user(Some("greefine"))
      .pass(Some("password"))
      .db_name(Some("Flowers"));
    Database {
      pool: mysql::Pool::new(database).unwrap(),
    }
  }

  pub fn get_flowers(&self, limit: i32) -> Flowers {
    let flower_desc: Vec<FlowerDesc> = self
      .pool
      .prep_exec(
        "SELECT nom_avec_auteur, genre FROM descriptions LIMIT :limit;",
        params! {
            "limit" => limit,
        },
      )
      .map(|result| {
        result
          .map(|x| x.unwrap())
          .map(|row| {
            let (id, name) = mysql::from_row(row);
            FlowerDesc { id: id, name: name }
          })
          .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
      })
      .unwrap(); // Unwrap `Vec<Payment>`


    Flowers {
      descrptions: flower_desc,
    }
  }
}

