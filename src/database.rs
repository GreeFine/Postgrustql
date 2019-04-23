use futures::Future;
use my::{OptsBuilder};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
  customer_id: i32,
  amount: i32,
  account_name: Option<String>,
}

/// Same as `tokio::run`, but will panic if future panics and will return the result
/// of future execution.
pub fn run<F, T, U>(future: F) -> Result<T, U>
where
  F: Future<Item = T, Error = U> + Send + 'static,
  T: Send + 'static,
  U: Send + 'static,
{
  let mut runtime = tokio::runtime::Runtime::new().unwrap();
  let result = runtime.block_on(future);
  runtime.shutdown_on_idle().wait().unwrap();
  result
}

pub fn start() -> my::Pool {
  let mut database = OptsBuilder::new();
  database
    .user(Some("greefine"))
    .pass(Some("password"))
    .db_name(Some("Flowers"));


  my::Pool::new(database)
}