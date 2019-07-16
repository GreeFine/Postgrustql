
use super::context::Context;
use super::types::{ConnectionDescription, ConnectionPicture, Description, User, UID};
use juniper::FieldResult;

pub struct Query;


juniper::graphql_object!(User: Context |&self| {
  field b(&executor, limit: Option<i32>) -> FieldResult<ConnectionPicture> {
    let db = &executor.context().database;
    db.request_objects(limit)
  }
  field a(&executor, limit: Option<i32>) -> FieldResult<ConnectionDescription> {
    let db = &executor.context().database;
    db.request_objects(limit)
  }
  field c(&executor, limit: Option<i32>) -> FieldResult<UID> {
    Ok(UID::default())
  }
});

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, limit: Option<i32>) -> FieldResult<User> {
      Ok(User::default())
    }

    field description(&executor) -> FieldResult<Description> {
      let db = &executor.context().database;
      db.request_object()
    }

    field pictures(&executor, limit: Option<i32>) -> FieldResult<ConnectionPicture> {
      let db = &executor.context().database;
      db.request_objects(limit)
    }

    field descriptions(&executor, limit: Option<i32>) -> FieldResult<ConnectionDescription> {
      let db = &executor.context().database;
      db.request_objects(limit)
    }
});
