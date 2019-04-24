use super::graphql::queries::{FlowerDesc, Flowers};
use juniper::GraphQLType;
use mysql::prelude::*;
use mysql::OptsBuilder;

pub struct Database {
  pool: mysql::Pool,
}

pub trait requestable_data {
  fn new() -> Self;
  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    panic!("not implem!")
  }
  fn create(row: &mut mysql::Row) -> Self;
}

#[derive(AsRefStr)]
pub enum e_tables {
  pictures,
  descriptions,
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

  pub fn request<T>(&self, fields: Vec<&str>, table: e_tables, limit: Option<u32>) -> T
  where
    T: requestable_data,
    T: GraphQLType,
  {
    let mut request = String::new();
    for field in &fields {
      request.push_str(field);
      request.push_str(", ");
    }
    request.truncate(request.len() - 2);
    match limit {
      None => request = format!("SELECT {0} FROM {1};", request, table.as_ref()),
      Some(x) => request = format!("SELECT {0} FROM {1} LIMIT {2};", request, table.as_ref(), x),
    }

    let mut data = T::new();
    let rows = self
      .pool
      .prep_exec(request, ())
      .map(|result| result.map(|x| x.unwrap()))
      .unwrap();
    for mut row in rows {
      requestable_data::feed(&mut data, &mut row);
    }

    data
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

