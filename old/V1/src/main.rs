use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
//use starwars::{QueryRoot, StarWars};

mod model;

//mod Something;

use model::*;

//mod memrs_type;

mod identifier;

mod stored_object;

mod stored_objects;

//mod value;

mod errors;

mod async_graphql_value_containers;

//mod non_generic_type_controller;

mod uniary_operations;

mod binary_operations;

mod ternary_operations;

//mod logic;

mod async_graphql_types;

mod all_types;

//mod instance_info;

mod value;

mod async_graphql_generic_type_info;

mod instance_type;

//mod task_actor_per_ns;

mod storage_container;

mod actor_utils;

mod builds;

mod store_traits;

use crate::builds::store_build::mutex::mtx_ns_scc::Store;

mod consts;

mod refcell_storage_container;

mod immut_value_storage_container;

mod storeage;

mod namespace_getters;

mod namespace_setters;

mod namespace_setters_mut;

mod mut_value_storage_container;

mod immut_mut_value_storage_container;



//mod generic_type_info;

//use memrs_type::Type;

//https://github.com/async-graphql/examples/blob/master/models/starwars/src/model.rs

//https://github.com/async-graphql/examples/blob/master/models/starwars/src/lib.rs

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Store::new())
        .finish();

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}

