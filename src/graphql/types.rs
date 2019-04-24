use super::super::database::ETables;
use juniper::GraphQLType;

pub struct Connection<T>
where
  T: juniper::GraphQLType,
{
  pub nodes: Vec<T>,
}

pub trait Connection_trait {
  fn new() -> Self;
  fn feed(&mut self, _: &mut mysql::Row) -> &mut Self {
    panic!("Not implem!")
  }
  fn create(_: &mut mysql::Row) -> Box<Self> {
    panic!("Not implemented!")
  }
}

impl Connection_trait for Picture {
  fn new() -> Self {
    Picture {
      binaire_href: String::new(),
      determination_ns: String::new(),
    }
  }
  fn create(row: &mut mysql::Row) -> Box<Self> {
    let mut _self = Self::new();
    _self.feed(row);
    Box::new(_self)
  }

  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    self.binaire_href = row.take(0).unwrap();
    self.determination_ns = row.take(1).unwrap();
    self
  }
}

impl<T> Connection_trait for Connection<T>
where
  T: Connection_trait,
  T: juniper::GraphQLType,
{
  fn new() -> Self {
    Connection { nodes: Vec::new() }
  }

  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    self.nodes.push(*T::create(row));
    self
  }
}

macro_rules! connection {
    ($v:ident) => (
      pub type PictureConnection = Connection<Picture>;
      juniper::graphql_object!(PictureConnection: () |&self| {
          field nodes() -> &Vec<$v> {
              &self.nodes
          }
      });
    );
}


#[derive(juniper::GraphQLObject, Debug)]
pub struct Picture {
  binaire_href: String,
  determination_ns: String,
}
connection!(Picture);


#[derive(juniper::GraphQLObject)]
#[graphql(description = "All the Flowers")]
pub struct Flowers {
  pub descrptions: Vec<FlowerDesc>,
}

#[derive(juniper::GraphQLObject)]
pub struct FlowerDesc {
  pub id: String,
  pub name: String,
}
