
use super::context::Context;
use super::types::{Description, DescriptionConnection, PictureConnection, User};
use juniper::FieldResult;

pub struct Query;

impl Query {
  pub fn init() {
    juniper::graphql_object!(Query: Context |&self| {
        field apiVersion() -> &str {
            "1.0"
        }

        field user(&executor, limit: Option<i32>) -> FieldResult<User> {
          Ok(User::default())
        }

        field description(&executor, limit: Option<i32>) -> FieldResult<Description> {
          let db = &executor.context().database;
          db.request_object()
        }

        field pictures(&executor, limit: Option<i32>) -> FieldResult<PictureConnection> {
          let db = &executor.context().database;
          db.request_objects(limit)
        }

        field descriptions(&executor, limit: Option<i32>) -> FieldResult<DescriptionConnection> {
          let db = &executor.context().database;
          db.request_objects(limit)
        }
    });
  }
}