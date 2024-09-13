mod builds;

mod types;

mod common_model_body_macros;

mod tasks_and_actors;

mod websocket_server;

use websocket_server::WebSocketServer ;

mod store;

pub use store::*;

mod errors;

#[tokio::main]
async fn main()
{

    WebSocketServer ::serve().await;

}
