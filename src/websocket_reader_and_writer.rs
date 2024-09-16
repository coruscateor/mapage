//Taken from Req It.

use std::future::Future;

use fastwebsockets::{FragmentCollectorRead, Frame, WebSocketError, WebSocketRead, WebSocketWrite};

use hyper::upgrade::Upgraded;

use hyper_util::rt::TokioIo;

use tokio::io::{ReadHalf, WriteHalf};



pub type WebSocketReadHalf = WebSocketRead<ReadHalf<TokioIo<Upgraded>>>;

pub type WebSocketWriteHalf = WebSocketWrite<WriteHalf<TokioIo<Upgraded>>>;

//#[derive(Debug)]
pub enum WebSocketReader
{

    WebSocketRead(WebSocketReadHalf),
    FragmentCollectorRead(FragmentCollectorRead<ReadHalf<TokioIo<Upgraded>>>)

}

impl WebSocketReader
{

    pub fn is_web_socket_reader(&self) -> bool
    {

        if let WebSocketReader::WebSocketRead(_) = self
        {
            
            return true;

        }

        false

    }

    pub fn is_fragment_collector_read(&self) -> bool
    {

        if let WebSocketReader::FragmentCollectorRead(_) = self
        {
            
            return true;

        }

        false

    }

    pub async fn read_frame<R, E>(&mut self, send_fn: &mut impl FnMut(Frame<'_>) -> R) -> Result<Frame<'_>, WebSocketError>
        where E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
              R: Future<Output = Result<(), E>>
    {

        match self
        {

            WebSocketReader::WebSocketRead(ws) =>
            {

                ws.read_frame(send_fn).await

            },
            WebSocketReader::FragmentCollectorRead(fc) =>
            {

                fc.read_frame(send_fn).await

            }

        }

    }

}