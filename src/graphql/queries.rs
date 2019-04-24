use super::super::database::{e_tables, requestable_data};
use super::context::Context;
use juniper::FieldResult;

pub struct Query;

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


#[derive(juniper::GraphQLObject, Debug)]
struct Picture {
  binaire_href: String,
  determination_ns: String,
}

#[derive(juniper::GraphQLObject, Debug)]
struct Pictures {
  pub nodes: Vec<Picture>,
}

impl requestable_data for Picture {
  fn new() -> Self {
    Picture {
      binaire_href: String::new(),
      determination_ns: String::new(),
    }
  }
  fn create(row: &mut mysql::Row) -> Self {
    let mut _self = Self::new();
    _self.feed(row);
    _self
  }

  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    self.binaire_href = row.take(0).unwrap();
    self.determination_ns = row.take(1).unwrap();
    self
  }
}


impl requestable_data for Pictures {
  fn new() -> Self {
    Pictures { nodes: Vec::new() }
  }

  fn create(_: &mut mysql::Row) -> Self {
    panic!("Not implem")
  }

  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    self.nodes.push(Picture::create(row));
    self
  }
}

impl Query {
  pub fn init() {
    juniper::graphql_object!(Query: Context |&self| {
        field apiVersion() -> &str {
            "1.0"
        }

        field flowers(&executor, limit: i32) -> FieldResult<Flowers> {
          let db = &executor.context().database;
          Ok(db.get_flowers(limit))
        }

        field pictures(&executor, limit: i32) -> FieldResult<Pictures> {
          let db = &executor.context().database;
          Ok(db.request::<Pictures>(
            vec!["binaire_href", "determination_ns"],
            e_tables::pictures,
            Some(limit as u32),
          ))
        }
    });
  }
}