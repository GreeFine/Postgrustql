use super::super::database::models::*;
use super::super::database::schema::users::dsl::*;
use super::context::Context;
use diesel::prelude::*;
use juniper::FieldResult;

pub struct Query;

juniper::graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field user(&executor, limit: Option<i32>) -> FieldResult<User> {
      let conn = &executor.context().db_link.connection();
      let results = users
          .limit(1)
          .load::<User>(conn)
          .expect("Error loading users");

      let res = results[0].clone();
      Ok(res)
    }
});
