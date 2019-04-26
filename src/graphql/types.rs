use super::super::database::ETables;
use super::context::Context;
use juniper::FieldResult;
use postgres::rows::Row;

pub trait RequestableObjects {
  fn field_names() -> &'static [&'static str];
  fn table() -> ETables;
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

pub trait ConnectionTrait {
  fn feed(&mut self, _: &mut Row) -> &mut Self {
    unimplemented!()
  }
  fn create(_: &mut Row) -> Box<Self> {
    unimplemented!()
  }
}

impl<T> ConnectionTrait for Connection<T>
where
  T: juniper::GraphQLType,
  T: RequestableObjects,
  T: Default,
{
  fn feed(&mut self, row: &mut Row) -> &mut Self {
    self.nodes.push(*T::row(row));
    self
  }
}

macro_rules! requestable_objects {
    ($table:ident struct $name:ident { $($fname:ident : $ftype:ty),* } ) => {
        #[derive(Default, Debug)]
        pub struct $name {
            $($fname : $ftype),*
        }

        juniper::graphql_object!($name: Context |&self| {
          $(
            field $fname() -> &$ftype {
                &self.$fname
            }
          ),*
        });

        #[allow(unused_assignments)]
        impl RequestableObjects for $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
            fn table() -> ETables {
              ETables::$table
            }
            fn row(row: &mut Row) -> Box<Self> {
              let mut obj_row = $name::default();
              let mut index = 0;
              $(
                obj_row.$fname = row.get(index);
                index += 1;
              )*
              Box::new(obj_row)
            }
        }
    }
}


macro_rules! objects_connection {
  ($conname:ident, $name:ident) => {
    pub type $conname = Connection<$name>;
    impl RequestableObjects for $conname
    where $name: RequestableObjects {
        fn field_names() -> &'static [&'static str] {
            $name::field_names()
        }
        fn table() -> ETables {
          $name::table()
        }
    }

    juniper::graphql_object!($conname: Context |&self| {
      field nodes() -> &Vec<$name> {
          &self.nodes
      }
    });
  }
}

requestable_objects! {
  pictures
  struct Picture {
    binaire_href: String,
    determination_ns: String,
    lieudit: String,
    pays: String
  }
}
objects_connection!(PictureConnection, Picture);


requestable_objects! {
  descriptions
  struct Description {
    nom_avec_auteur: String,
    num_taxonomique: String,
    annee_et_bibliographie: String,
    nom_commercial: String
  }
}
objects_connection!(DescriptionConnection, Description);

#[derive(Debug, Default)]
pub struct User {
  a: DescriptionConnection,
  b: PictureConnection,
}

juniper::graphql_object!(User: Context |&self| {
  field b(&executor, limit: Option<i32>) -> FieldResult<PictureConnection> {
    let db = &executor.context().database;
    db.request_objects(limit)
  }
  field a(&executor, limit: Option<i32>) -> FieldResult<DescriptionConnection> {
    let db = &executor.context().database;
    db.request_objects(limit)
  }
});