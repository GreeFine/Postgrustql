use super::graphql::types::{ConnectionTrait, RequestableObjects};
use juniper::FieldResult;
use juniper::GraphQLType;
use mysql::OptsBuilder;

pub struct Database {
  pool: mysql::Pool,
}


#[derive(AsRefStr)]
#[allow(non_camel_case_types)]
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

  pub fn request_objects<T>(&self, limit: Option<i32>) -> FieldResult<T>
  where
    T: GraphQLType,
    T: Default,
    T: RequestableObjects,
    T: ConnectionTrait,
  {
    let mut request = String::new();
    let mut data = T::default();
    let table = T::table();
    let fields = T::field_names();

    for field in fields {
      request.push_str(field);
      request.push_str(", ");
    }
    request.truncate(request.len() - 2);
    match limit {
      None => request = format!("SELECT {0} FROM {1};", request, table.as_ref()),
      Some(x) => request = format!("SELECT {0} FROM {1} LIMIT {2};", request, table.as_ref(), x),
    }

    println!("SQL CONNECTION: {0}", request);
    let rows = self
      .pool
      .prep_exec(request, ())
      .map(|result| result.map(|x| x.unwrap()))
      .unwrap();
    for mut row in rows {
      data.feed(&mut row);
    }

    Ok(data)
  }

  pub fn request_object<T>(&self) -> FieldResult<T>
  where
    T: GraphQLType,
    T: Default,
    T: RequestableObjects,
  {
    //FIXME: What if this is a multi result
    let mut request = String::new();
    let table = T::table();
    let fields = T::field_names();

    for field in fields {
      request.push_str(field);
      request.push_str(", ");
    }
    request.truncate(request.len() - 2);
    request = format!("SELECT {0} FROM {1} LIMIT 1;", request, table.as_ref());

    println!("SQL OBJECT: {0}", request);
    let rows = self
      .pool
      .prep_exec(request, ())
      .map(|result| result.map(|x| x.unwrap()))
      .unwrap();
    let mut obj = T::default();
    for mut row in rows {
      obj = *T::row(&mut row);
      break;
    }
    Ok(obj)
  }
}

