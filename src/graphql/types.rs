use super::context::Context;
use postgres::rows::Row;
use requestable::{objects_connection, requestable_object};

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
    nom_commercial: String
    // desc: Picture
  }
}

objects_connection!(Description);

#[derive(Debug, Default)]
pub struct User {
  a: ConnectionDescription,
  b: ConnectionPicture,
}