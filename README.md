# Motivation 

Simplify use of `graphql_client` when developing for both non-blocking (wasm) and blocking targets.

# Synopsis

Examples & better documentation will come later if necessary. For now:

Make sure the following dependencies are in Cargo.toml:

```
graphql_client = { version = ">=0.11.0" }
graphql_client_simply = { path = "graphql_client_simply" }
```

In your code: 

```
// Import the 
use graphql_client_simply::{gqlquery, GqlPromiseData, GraphQLQuery};

// Derive Rust types for your queries, as you normally would with `graphql_client`. 
// See https://github.com/graphql-rust/graphql-client for more details.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rickandmorty.schema.json",
    query_path = "graphql/eg_chars_query.graphql",
    response_derives = "Debug"
)]
pub struct EgCharsQuery;


// Have some stateful (particularly for async situations) location. Below is just an
// example -- you have to set up your own state somehow.
struct State {
    chars_query: GqlPromiseData<EgCharsQuery>,
}


// Do the query and store the resulting data / promise (GqlPromiseData) in your stateful
// location. 
let state.chars_query = gqlquery::<EgCharsQuery>(url, variables);


// Somewhere else in your app, get the data for usage
if state.chars_query.is_fetching() {
    println!("Fetching...");
} else {
    if let Some(data) = state.chars_query.get() {
        println!("Query results: {:#?}", data);
    }
}

// You can clear the data / promise as follows:
state.chars_query.clear();



```