
use super::context::Context;
use juniper::FieldResult;
pub struct Query;

#[derive(juniper::GraphQLEnum)]
pub enum Episode {
  NewHope,
  Empire,
  Jedi,
}

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
  pub id: String,
  pub name: String,
  pub appears_in: Vec<Episode>,
  pub home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

juniper::graphql_object!(Query: Context |&self| {

    field apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // The executor is a special (optional) argument that allows accessing the context.
    field human(&executor, id: String) -> FieldResult<Human> {
        // Get the context from the executor.
        let context = executor.context();
        // Get a db connection.
        // let connection = context.pool.get_connection()?;
        // Execute a db query.
        // Note the use of `?` to propagate errors.
        // let human = connection.find_human(&id)?;
        // Return the result.
        let new_human = Human{
            id: String::from("r2"),
            name: String::from("d2"),
            appears_in: Vec::new(),
            home_planet: String::from("terre"),
        };
        Ok(new_human)
    }
});