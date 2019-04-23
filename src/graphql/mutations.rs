// use juniper::FieldResult;
// use super::queries::{Human, NewHuman};
use super::context::Context;

pub struct Mutation;

juniper::graphql_object!(Mutation: Context |&self| {

//     field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
//         // let db = executor.context().pool.get_connection()?;
//         // let human: Human = db.insert_human(&new_human)?;
//         let new_human = Human{
//             id: String::from("r2"),
//             name: String::from("d2"),
//             appears_in: Vec::new(),
//             home_planet: String::from("terre"),
//         };
//         Ok(new_human)
//     }
});