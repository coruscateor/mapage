use axum::{extract::State, response::IntoResponse, routing::get, Router};

use fastwebsockets::{upgrade, OpCode, WebSocketError, FragmentCollector};

use tokio::{net::TcpListener, task};

use upgrade::{UpgradeFut, IncomingUpgrade};

use std::sync::Arc;

use crate::{SimpleWebSocketPipeline, Store};

pub struct WebSocketServer ();

//https://docs.rs/axum/latest/axum/#sharing-state-with-handlers

impl WebSocketServer 
{

    pub async fn serve()
    {

        let store = Arc::new(Store::new());

        let method_router = get(ws_handler).with_state(store);

        let app = Router::new().route("/", method_router);

        //let app = Router::new().route("/", get(ws_handler)).with_state(store);
    
        //Make the port number a variable.

        let listener;

        match TcpListener::bind("0.0.0.0:3000").await
        {

            Ok(new_listener) =>
            {

                listener = new_listener;

            }
            Err(err) =>
            {

                //Output to console only for now...

                println!("{}", err);

                return;

            } 
        }
    
        println!("Mapage is listening on: localhost:3000");

        match axum::serve(listener, app).await
        {

            Ok(_) =>
            {

                println!("bye bye");

            }
            Err(err) =>
            {

                println!("{}", err);

            }

        }

    }

}

async fn handle_client(fut: UpgradeFut) -> Result<(), WebSocketError>
{

    let mut ws = FragmentCollector::new(fut.await?);

    loop
    {

        let frame = ws.read_frame().await?;

        match frame.opcode
        {

            OpCode::Close =>
            {

                println!("Close frame received.");

                break;

            }
            OpCode::Text | OpCode::Binary =>
            {

                ws.write_frame(frame).await?;

            },
            _ => {

                println!("I should do something now.")

            }
            
        }
        
    }

    Ok({})

}

async fn ws_handler(ws: IncomingUpgrade, State(store): State<Arc<Store>>) -> impl IntoResponse //State(store): State<Store>,
{

    let (response, fut) = ws.upgrade().unwrap();

    SimpleWebSocketPipeline::new( fut, store);

    /*
    task::spawn(async move {

        if let Err(e) = handle_client(fut).await
        {

            eprintln!("Error in websocket connection: {}", e);

        }

    });
    */

    //Maybe log the response somewhere.

    //println!("{}", response);

    response

}