pub use super::queries::Query;
pub use super::context::Context;
pub use super::mutations::Mutation;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;