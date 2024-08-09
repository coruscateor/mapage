use websockets_server::WebSocketsServer;

mod builds;

mod types;

mod common_model_body_macros;

mod tasks_and_actors;

mod websockets_server;

#[tokio::main]
async fn main()
{

    WebSocketsServer::serve().await;

}
