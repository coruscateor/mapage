use axum::{response::IntoResponse, routing::get, Router};

use fastwebsockets::{upgrade, OpCode, WebSocketError, FragmentCollector};

use tokio::{net::TcpListener, task};

use upgrade::{UpgradeFut, IncomingUpgrade};

pub struct WebSocketsServer();

impl WebSocketsServer
{

    pub async fn serve()
    {

        let app = Router::new().route("/", get(ws_handler));

        //println!("app\n");
    
        //println!("{app:?}\n");
    
        //let listener = TcpListener::bind("localhost:3000").await.unwrap(); //TcpListener::bind("http://localhost:3000").await.unwrap(); //TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
        //println!("listener\n");
    
        //println!("{listener:?}\n");
    
        println!("Mapage is listening on: localhost:3000"); //ws://
    
        axum::serve(listener, app).await.unwrap();

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

            } //break,
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

    response

}