mod builds;

mod types;

mod common_model_body_macros;

mod tasks_and_actors;

mod websocket_server;

use websocket_server::WebSocketServer ;

mod store;

pub use store::*;

mod errors;

mod websocket_actors;

mod simple_websocket_pipeline;

pub use simple_websocket_pipeline::*;

mod websocket_reader_and_writer;

pub use websocket_reader_and_writer::*;

#[tokio::main]
async fn main()
{

    WebSocketServer ::serve().await;

}
