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

mod communication_instance;

pub use communication_instance::*;

mod command_error;

pub use command_error::*;

#[tokio::main]
async fn main()
{

    WebSocketServer::serve().await;

}
