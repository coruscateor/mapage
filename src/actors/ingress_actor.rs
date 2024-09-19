use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::OwnedFrame;

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::json;

use super::array_queue::ActorIOClient;

pub struct IngressActorState
{

    websocket_actor_io_client: ActorIOClient<OwnedFrame, OwnedFrame>
    //actor_io_reciver: Receiver<OwnedFrame>

}

impl IngressActorState
{

    pub fn new(websocket_actor_io_client: &ActorIOClient<OwnedFrame, OwnedFrame>) -> Self //actor_io_reciver: Receiver<OwnedFrame>) -> Self
    {

        Self
        {

            websocket_actor_io_client: websocket_actor_io_client.clone()
            //actor_io_reciver

        }

    }

    pub fn spawn() //-> Receiver<()>
    {



    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        if let Some(mut res) = self.websocket_actor_io_client.output_receiver().recv().await //actor_io_reciver.recv().await
        {

            match res.opcode
            {

                OpCode::Continuation =>
                {

                    print!("{}", "Continuation frames are not allowed.");

                    return false;

                }
                OpCode::Text =>
                {

                    

                }
                OpCode::Binary =>
                {

                    let error_message = r#"{
                                                    "error": "The binary OpCode is not supported."
                                                }"#;

                    res.text_setup();

                    res.set_payload_from_str(error_message);

                    if let Err(_err) = self.websocket_actor_io_client.input_sender().send(res).await
                    {

                        return false;

                    }

                }
                OpCode::Close =>
                {

                    print!("{}", "Close frames are not allowed.");

                    return false;

                }
                OpCode::Ping =>
                {

                    print!("{}", "Ping frames are not allowed.");

                    return false;

                }
                OpCode::Pong =>
                {

                    print!("{}", "Pong frames are not allowed.");

                    return false;

                }

            }

        }
        else
        {

            return false;
            
        }

        true

    }

}

impl_mac_task_actor!(IngressActor);