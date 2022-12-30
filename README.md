# Motivation 

Simplify use of the [graphql_client](https://github.com/graphql-rust/graphql-client) crate when developing for wasm targets and otherwise. That is, for `cfg(target_arch = "wasm32")` this will use non-blocking GraphQL calls, and for `cfg(not(target_arch = "wasm32"))` it will use blocking calls.

Obviously this is a pretty specific approach, so the crate may not be suitable for all projects. I am currently using it in a [notan](https://github.com/Nazariglez/notan) + [egui](https://github.com/emilk/egui) project and it works well for that.

To manage async calls in wasm targets, the crate uses [poll-promise](https://github.com/EmbarkStudios/poll-promise).

Examples & better documentation will come later if necessary/on request. For now:


# Synopsis


Make sure the following dependencies are in Cargo.toml:

```
graphql_client = { version = ">=0.11.0" }
graphql_client_simply = { git = "https://github.com/riverfr0zen/graphql_client_simply.git" }
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

let state = State {
    chars_query: None,
}


//
// Do the query and store the resulting data / promise (GqlPromiseData) in your stateful
// location. 
//
let variables = eg_chars_query::Variables {};
let url = "https://rickandmortyapi.com/graphql";
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