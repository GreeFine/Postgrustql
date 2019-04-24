
use super::super::database::ETables;
use super::context::Context;
use super::types::PictureConnection;
use super::types::*;
use juniper::FieldResult;
pub struct Query;

impl Query {
  pub fn init() {
    juniper::graphql_object!(Query: Context |&self| {
        field apiVersion() -> &str {
            "1.0"
        }

        field pictures(&executor, limit: Option<i32>) -> FieldResult<PictureConnection> {
          let db = &executor.context().database;
          Ok(db.request::<PictureConnection>(
            vec!["binaire_href", "determination_ns"],
            ETables::pictures,
            limit,
          ))
        }
    });
  }
}