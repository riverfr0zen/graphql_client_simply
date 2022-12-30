///
/// An attempt to simplify use of `graphql_client` when developing for both
/// non-blocking (wasm) and blocking targets.
///
/// Using these examples as reference:
/// https://github.com/graphql-rust/graphql-client/tree/main/examples/github
///
///
pub use graphql_client::GraphQLQuery;
use log;
use poll_promise::Promise;
use std::task::Poll;
#[cfg(target_arch = "wasm32")]
use {graphql_client::reqwest::post_graphql, reqwest::Client};
#[cfg(not(target_arch = "wasm32"))]
use {graphql_client::reqwest::post_graphql_blocking as post_graphql, reqwest::blocking::Client};


// #[cfg(target_arch = "wasm32")]
type QueryPromise<T> = Option<Promise<Option<T>>>;


pub struct GqlPromiseData<Q>
where
    Q: GraphQLQuery,
    <Q as GraphQLQuery>::ResponseData: Send + 'static,
{
    data: Option<Q::ResponseData>,
    promise: QueryPromise<Q::ResponseData>,
}


impl<Q> Default for GqlPromiseData<Q>
where
    Q: GraphQLQuery,
    <Q as GraphQLQuery>::ResponseData: Send,
{
    fn default() -> Self {
        Self {
            data: None,
            promise: None,
        }
    }
}


#[cfg(target_arch = "wasm32")]
pub async fn _gqlquery<Q>(url: &str, variables: Q::Variables) -> Option<Q::ResponseData>
where
    Q: GraphQLQuery,
{
    let client = Client::new();
    let response_body = post_graphql::<Q, _>(&client, url, variables)
        .await
        .map_err(|err| {
            log::error!("{:?}", err);
        })
        .unwrap();
    // log::debug!("{:#?}", response_body);
    if let Some(response_data) = response_body.data {
        Some(response_data)
    } else {
        None
    }
}


#[cfg(not(target_arch = "wasm32"))]
pub fn _gqlquery<Q>(url: &str, variables: Q::Variables) -> Option<Q::ResponseData>
where
    Q: GraphQLQuery,
{
    // GraphQL stuff based on
    // https://github.com/graphql-rust/graphql-client/tree/main/examples/github
    let client = Client::new();
    let response_body = post_graphql::<Q, _>(&client, url, variables)
        .map_err(|err| {
            log::error!("{:?}", err);
        })
        .unwrap();
    // log::debug!("{:#?}", response_body);
    if let Some(response_data) = response_body.data {
        Some(response_data)
    } else {
        None
    }
}


pub fn gqlquery<Q>(url: &'static str, variables: Q::Variables) -> GqlPromiseData<Q>
where
    Q: GraphQLQuery,
    <Q as GraphQLQuery>::ResponseData: Send,
    <Q as GraphQLQuery>::Variables: Send + 'static,
{
    log::debug!("Running query {}", std::any::type_name::<Q>());

    #[cfg(target_arch = "wasm32")]
    {
        let promise = Promise::spawn_async(async move { _gqlquery::<Q>(url, variables).await });
        GqlPromiseData {
            promise: Some(promise),
            data: None,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        GqlPromiseData {
            promise: None,
            data: _gqlquery::<Q>(&url, variables),
        }
    }
}


impl<Q> GqlPromiseData<Q>
where
    Q: GraphQLQuery,
    <Q as GraphQLQuery>::ResponseData: Send,
{
    pub fn get(&self) -> &Option<Q::ResponseData> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(promise) = &self.promise {
                // Use/show result
                if let Some(val) = promise.ready() {
                    log::debug!("Returning data for {}", std::any::type_name::<Q>());
                    return val;
                } else {
                    log::debug!("{}...waiting for response", std::any::type_name::<Q>());
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(_) = &self.data {
                log::debug!("Returning data for {}", std::any::type_name::<Q>());
                return &self.data;
            }
        }

        &None
    }


    pub fn clear(&mut self) {
        self.data = None;
        self.promise = None;
        log::debug!("Cleared promise / data for {}", std::any::type_name::<Q>());
    }

    pub fn is_fetching(&self) -> bool {
        if let Some(promise) = &self.promise {
            match promise.poll() {
                Poll::Ready(_) => return false,
                Poll::Pending => return true,
            }
        }
        false
    }
}
