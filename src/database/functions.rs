use diesel::pg::PgConnection;
pub use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;

pub struct DbLink(Pool<ConnectionManager<PgConnection>>);

impl DbLink {
  pub fn pool_create() -> Self {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Self(Pool::new(manager).unwrap())
  }

  pub fn connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
    self.0.get().unwrap()
  }
}
