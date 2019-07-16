use super::super::database::functions::DbLink;

pub struct Context {
  pub db_link: DbLink, // Use your real database pool here.
}

impl Context {
  pub fn new() -> Context {
    Context {
      db_link: DbLink::pool_create(),
    }
  }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}
