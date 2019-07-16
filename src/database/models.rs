#[derive(Clone, Queryable, juniper::GraphQLObject)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
}

use super::schema::users;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub name: &'a str,
  pub password: &'a str,
  pub email: &'a str,
}
