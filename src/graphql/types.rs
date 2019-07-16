use super::context::Context;
use postgres::rows::Row;
use postgres::types::FromSql;
use postgres::types::Type;
use requestable::{objects_connection, requestable_object};
use std::error::Error;
use std::str::from_utf8;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct UID(Uuid);
juniper::graphql_object!(UID: Context |&self| {
    field UID() -> &UID {
        &self
    }
});

impl FromSql for UID {
  fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
    Ok(UID(Uuid::parse_str(from_utf8(raw).unwrap()).unwrap()))
  }
  fn accepts(ty: &Type) -> bool {
    false
  }
}

// use juniper::{graphql_scalar, ParseScalarResult, ParseScalarValue};
// graphql_scalar!(UserID {
//     description: "An opaque identifier, represented as a string"

//     resolve(&self) -> UserID {
//         &self.0
//     }

//     from_input_value(v: &InputValue) -> Option<UserID> {
//         v.as_scalar_value().map(|x: &String| UserID(Uuid::parse_str(x).unwrap()))
//     }

//     from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a> {
//         <Uuid as ParseScalarValue>::from_str(value)
//     }
// });

pub trait RequestableObject {
  fn field_names() -> &'static [&'static str];
  fn table() -> &'static str;
  fn row(_row: &mut Row) -> Box<Self> {
    unimplemented!()
  }
}

#[derive(Default, Debug)]
pub struct Connection<T>
where
  T: juniper::GraphQLType,
{
  pub nodes: Vec<T>,
}

pub trait ConnectionTrait<T, X>
where
  X: RequestableObject,
{
  fn feed(&mut self, row: &mut Row) -> &mut Self;
  fn create(_: &mut Row) -> Box<Self> {
    unimplemented!()
  }
  fn field_names() -> &'static [&'static str] {
    X::field_names()
  }
  fn table() -> &'static str {
    X::table()
  }
}

//X == Picture // T == PictureConnection
impl<T, X> ConnectionTrait<T, X> for Connection<X>
where
  T: juniper::GraphQLType,
  X: juniper::GraphQLType,
  X: RequestableObject,
  X: Default,
{
  fn feed(&mut self, row: &mut Row) -> &mut Self {
    self.nodes.push(*X::row(row));
    self
  }
}

requestable_object! {
  "pictures"
  pub struct Picture {
    binaire_href: String,
    determination_ns: String,
    lieudit: String,
    pays: String
  }
}
objects_connection!(Picture);

requestable_object! {
  "descriptions"
  pub struct Description {
    nom_avec_auteur: String,
    num_taxonomique: String,
    annee_et_bibliographie: String,
    nom_commercial: String,
  }
}

objects_connection!(Description);

#[derive(Debug, Default)]
pub struct User {
  a: ConnectionDescription,
  b: ConnectionPicture,
  c: UID,
}