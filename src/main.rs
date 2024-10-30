mod builds;

mod types;

mod common_model_body_macros;

mod tasks_and_actors;

mod websocket_server;

use websocket_server::WebSocketServer ;

mod store;

pub use store::*;

mod errors;

mod actors;

mod simple_websocket_pipeline;

pub use simple_websocket_pipeline::*;

mod websocket_reader_and_writer;

pub use websocket_reader_and_writer::*;

mod owned_frame;

pub use owned_frame::*;

mod command_result;

pub use command_result::*;

#[tokio::main]
async fn main()
{

    WebSocketServer::serve().await;

}
