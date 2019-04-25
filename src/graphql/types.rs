use super::super::database::ETables;

#[derive(Default)]
pub struct Connection<T>
where
  T: juniper::GraphQLType,
{
  pub nodes: Vec<T>,
}

pub trait ConnectionTrait {
  fn feed(&mut self, _: &mut mysql::Row) -> &mut Self {
    unimplemented!()
  }
  fn create(_: &mut mysql::Row) -> Box<Self> {
    unimplemented!()
  }
}

impl<T> ConnectionTrait for Connection<T>
where
  T: ConnectionTrait,
  T: juniper::GraphQLType,
{
  fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
    self.nodes.push(*T::create(row));
    self
  }
}

pub trait RequestableObjects {
  fn table(&self) -> ETables;
  fn field_names(&self) -> &'static [&'static str];
}

macro_rules! requestable_objects {
    ($conname:ident $table:ident struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(juniper::GraphQLObject, Debug, Default)]
        pub struct $name {
            $($fname : $ftype),*
        }

        impl ConnectionTrait for $name {
          fn create(row: &mut mysql::Row) -> Box<Self> {
            let mut _self = Self::default();
            _self.feed(row);
            Box::new(_self)
          }

          #[allow(unused_assignments)]
          fn feed(&mut self, row: &mut mysql::Row) -> &mut Self {
            let mut index = 0;
            $(
              self.$fname = row.take(index).unwrap();
              index+=1;
            )*
            self
          }
        }

        pub type $conname = Connection<$name>;
          juniper::graphql_object!($conname: () |&self| {
          field nodes() -> &Vec<$name> {
              &self.nodes
          }
        });

        impl RequestableObjects for $conname {
            fn field_names(&self) -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
            fn table(&self) -> ETables {
              ETables::$table
            }
        }
    }
}

requestable_objects! {
  PictureConnection
  pictures
  struct Picture {
    binaire_href: String,
    determination_ns: String,
    lieudit: String,
    pays: String
  }
}

requestable_objects! {
  DescriptionConnection
  descriptions
  struct Description {
    nom_avec_auteur: String,
    num_taxonomique: String,
    annee_et_bibliographie: String,
    nom_commercial: String
  }
}