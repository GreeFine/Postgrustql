use super::context::Context;
use juniper::FieldResult;

pub struct Query;

#[derive(juniper::GraphQLObject)]
#[graphql(description = "All the Flowers")]
pub struct Flowers {
  pub descrptions: Vec<FlowerDesc>,
}

#[derive(juniper::GraphQLObject)]
pub struct FlowerDesc {
  pub id: String,
  pub name: String,
}

impl Query {
  pub fn init() {
    juniper::graphql_object!(Query: Context |&self| {
        field apiVersion() -> &str {
            "1.0"
        }

        field flowers(&executor, limit: i32) -> FieldResult<Flowers> {
          let db = &executor.context().database;
          Ok(db.get_flowers(limit))
        }
    });
  }
}