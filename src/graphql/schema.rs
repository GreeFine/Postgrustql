pub use super::queries::Query;
pub use super::context::Context;
// pub use super::mutations::Mutation;
use juniper::EmptyMutation;

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;