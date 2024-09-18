use crate::OwnedFrame;

#[derive(Debug)]
pub enum SimpleWebSocketActorInputMessage
{

    Disconnect,
    WriteFrame(OwnedFrame)

}

