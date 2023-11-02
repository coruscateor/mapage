use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server, EndpointExt};

use poem::{http::Method, middleware::Cors};

mod builds;

mod types;

mod errors;

mod common_model_body_macros;

mod resolver_objects;

use resolver_objects::{QueryRoot, StoreType, MutationRoot};

use crate::resolver_objects::new_store;

mod tasks_and_actors;

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main()
{

    let store = new_store();

    let schema = Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(store)
        .finish();

    //Cors - Will be made optional

    //Cors middleware is requred if you want browser based applications to interact with Mapage directly.

    let cors = Cors::new();

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)).with(cors));

    println!("Playground: http://localhost:8000");
    
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}

