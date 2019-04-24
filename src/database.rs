use super::graphql::types::Connection_trait;
use juniper::GraphQLType;
use mysql::prelude::*;
use mysql::OptsBuilder;

pub struct Database {
  pool: mysql::Pool,
}


#[derive(AsRefStr)]
pub enum ETables {
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

  pub fn request<T>(&self, fields: Vec<&str>, table: ETables, limit: Option<i32>) -> T
  where
    T: Connection_trait,
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
      Connection_trait::feed(&mut data, &mut row);
    }

    data
  }
}

