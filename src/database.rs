use super::graphql::types::{ConnectionTrait, RequestableObjects};
use juniper::FieldResult;
use juniper::GraphQLType;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

#[derive(AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ETables {
  pictures,
  descriptions,
}

pub struct DbConnection(Pool<PostgresConnectionManager>);

impl DbConnection {
  pub fn pool_connect() -> DbConnection {
    let manager = PostgresConnectionManager::new(
      "postgres://greefine:password@localhost:5432/flowers",
      TlsMode::None,
    )
    .unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();
    Self(pool)
  }

  fn client(&self) -> PooledConnection<PostgresConnectionManager> {
    self.0.get().unwrap()
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
    let rows = &self.client().query(&request, &[]).unwrap();
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
    let rows = &self.client().query(&request, &[]).unwrap();
    let mut obj = T::default();
    for mut row in rows {
      obj = *T::row(&mut row);
      break;
    }
    Ok(obj)
  }
}

