use super::super::database::Database;

pub struct Context {
  pub database: Database
    // Use your real database pool here.
}
impl Context {
  pub fn new() -> Context {
    Context { database: Database::new()}
  }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}