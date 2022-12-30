# Motivation 

Simplify use of the [graphql_client](https://github.com/graphql-rust/graphql-client) crate when developing for both non-blocking (wasm) and blocking targets.

This crate is in early dev for a specific (gaming/graphics related) project, so it won't be entirely flexible in terms of how async is handled. For example, it uses [poll-promise](https://github.com/EmbarkStudios/poll-promise) crate, which may not be suitable for your application.

Examples & better documentation will come later if necessary/on request. For now:


# Synopsis


Make sure the following dependencies are in Cargo.toml:

```
graphql_client = { version = ">=0.11.0" }
graphql_client_simply = { path = "graphql_client_simply" }
```

In your code: 

```
//
// Import the items we need
//
use graphql_client_simply::{gqlquery, GqlPromiseData, GraphQLQuery};

//
// Derive Rust types for your queries, as you normally would with `graphql_client`. 
// See https://github.com/graphql-rust/graphql-client for more details.
//
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/rickandmorty.schema.json",
    query_path = "graphql/eg_chars_query.graphql",
    response_derives = "Debug"
)]
pub struct EgCharsQuery;


//
// Have some stateful (particularly for async situations) location. Below is just an
// example -- you have to set up your own state somehow.
//
struct State {
    chars_query: GqlPromiseData<EgCharsQuery>,
}


//
// Do the query and store the resulting data / promise (GqlPromiseData) in your stateful
// location. 
//
let state.chars_query = gqlquery::<EgCharsQuery>(url, variables);


//
// Somewhere else in your app (like an update or render loop) get the data for usage, if
// it is available
//
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