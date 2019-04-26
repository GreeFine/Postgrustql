use super::super::database::DbConnection;

pub struct Context {
  pub database: DbConnection, // Use your real database pool here.
}
impl Context {
  pub fn new() -> Context {
    Context {
      database: DbConnection::pool_connect(),
    }
  }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}