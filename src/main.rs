use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
//use starwars::{QueryRoot, StarWars};

//mod model;


//use model::*;

mod builds;

//#[cfg(store_aml)]

//cfg_if::cfg_if! {
//    if #[cfg(feature = "sub_store_aml")] {

//#[cfg(feature = "sub_store_aml")] //feature = 
//use crate::builds::levels::sub_namespace::store::Store;

//type StoreType = Store;

//    }
//}

mod types;

//mod macros;

//#[cfg(feature = "sub_store_aml")]
mod errors;

//mod macros;

//mod mutation_model_macros;

//mod query_model_macros;

//mod common_model_macros;

mod common_model_body_macros;

mod resolver_objects;

use resolver_objects::{QueryRoot, StoreType, MutationRoot};

use crate::resolver_objects::new_store; //MutationType

mod tasks_and_actors;

//https://github.com/async-graphql/examples/blob/master/models/starwars/src/model.rs

//https://github.com/async-graphql/examples/blob/master/models/starwars/src/lib.rs

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main()
{

    let store = new_store(); //StoreType::new();

    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription) //MutationType
        //#[cfg(not(any(feature = "all_types", feature = "SelectedTypeOI")))]
        //.data(StoreType::new())
        .data(store)
        .finish();

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");
    
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}

