use futures::Future;
use my::prelude::*;
use my::{Opts, OptsBuilder};

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

pub fn start() {
  let mut database = OptsBuilder::new();
  database
    .user(Some("greefine"))
    .pass(Some("password"))
    .db_name(Some("Flowers"));

  let payments = vec![
    Payment {
      customer_id: 1,
      amount: 2,
      account_name: None,
    },
    Payment {
      customer_id: 3,
      amount: 4,
      account_name: Some("foo".into()),
    },
    Payment {
      customer_id: 5,
      amount: 6,
      account_name: None,
    },
    Payment {
      customer_id: 7,
      amount: 8,
      account_name: None,
    },
    Payment {
      customer_id: 9,
      amount: 10,
      account_name: Some("bar".into()),
    },
  ];
  let payments_clone = payments.clone();

  let pool = my::Pool::new(database);
  let future = pool
    .get_conn()
    .and_then(|conn| {
      // Create temporary table
      conn.drop_query(
        r"CREATE TABLE payment (
                customer_id int not null,
                amount int not null,
                account_name text
            )",
      )
    })
    .and_then(move |conn| {
      // Save payments
      let params = payments_clone.into_iter().map(|payment| {
        params! {
            "customer_id" => payment.customer_id,
            "amount" => payment.amount,
            "account_name" => payment.account_name.clone(),
        }
      });

      conn.batch_exec(
        r"INSERT INTO payment (customer_id, amount, account_name)
                        VALUES (:customer_id, :amount, :account_name)",
        params,
      )
    })
    .and_then(|conn| {
      // Load payments from database.
      conn.prep_exec("SELECT customer_id, amount, account_name FROM payment", ())
    })
    .and_then(|result| {
      // Collect payments
      result.map_and_drop(|row| {
        let (customer_id, amount, account_name) = my::from_row(row);
        Payment {
          customer_id: customer_id,
          amount: amount,
          account_name: account_name,
        }
      })
    })
    .and_then(|(_ /* conn */, payments)| {
      // The destructor of a connection will return it to the pool,
      // but pool should be disconnected explicitly because it's
      // an asynchronous procedure.
      pool.disconnect().map(|_| payments)
    });

  let loaded_payments = run(future).unwrap();
  assert_eq!(loaded_payments, payments);
}