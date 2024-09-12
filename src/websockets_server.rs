use axum::{response::IntoResponse, routing::get, Router};

use fastwebsockets::{upgrade, OpCode, WebSocketError, FragmentCollector};

use tokio::{net::TcpListener, task};

use upgrade::{UpgradeFut, IncomingUpgrade};

pub struct WebSocketsServer();

//https://docs.rs/axum/latest/axum/#sharing-state-with-handlers

impl WebSocketsServer
{

    pub async fn serve()
    {

        let app = Router::new().route("/", get(ws_handler));
    
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

async fn ws_handler(ws: IncomingUpgrade) -> impl IntoResponse
{

    let (response, fut) = ws.upgrade().unwrap();

    task::spawn(async move {

        if let Err(e) = handle_client(fut).await
        {

            eprintln!("Error in websocket connection: {}", e);

        }

    });

    //Maybe log the response somewhere.

    //println!("{}", response);

    response

}