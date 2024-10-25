use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::OwnedFrame;

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, ParsedInput};

//Converts the OwnedFrames payload into an in-memory representation of the desired format and passes it on. 

pub struct IngressActorState
{

    websocket_actor_io_client: ActorIOClient<OwnedFrame, OwnedFrame>,
    command_processor_sender: Sender<ParsedInput>
    //egress_actor_sender

}

impl IngressActorState
{

    pub fn new(websocket_actor_io_client: &ActorIOClient<OwnedFrame, OwnedFrame>, command_processor_sender: Sender<ParsedInput>) -> Self //actor_io_reciver: Receiver<OwnedFrame>) -> Self
    {

        Self
        {

            websocket_actor_io_client: websocket_actor_io_client.clone(),
            command_processor_sender

        }

    }

    pub fn spawn(websocket_actor_io_client: &ActorIOClient<OwnedFrame, OwnedFrame>, command_processor_sender: Sender<ParsedInput>) //-> Receiver<()>
    {

        IngressActor::spawn(IngressActorState::new(websocket_actor_io_client, command_processor_sender));

    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(mut of_res) = self.websocket_actor_io_client.output_receiver().recv().await //actor_io_reciver.recv().await
        {

            match of_res.opcode
            {

                OpCode::Continuation =>
                {

                    print!("{}", "Continuation frames are not allowed.");

                    return false;

                }
                OpCode::Text =>
                {

                    let con_res = str::from_utf8(&of_res.payload);

                    match con_res
                    {

                        Ok(res) =>
                        {

                            let json_res = from_str::<Value>(res);

                            match json_res
                            {

                                Ok(json_obj) =>
                                {

                                    if let Err(_err) = self.command_processor_sender.send(json_obj).await
                                    {

                                        return false;

                                    }

                                    //Cache of_res

                                }
                                Err(err) =>
                                {

                                    //Should probabaly go through the EgressActor...

                                    let error_message = format!(r#"
                                       {{ "error": "{}" }}
                                    "#, err);

                                    of_res.text_setup();

                                    of_res.set_payload_from_str(&error_message);

                                    if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                                    {

                                        return false;

                                    }

                                }

                            }

                        }
                        Err(err) =>
                        {

                            let error_message = format!(r#"
                                {{ "error": "{}" }}
                            "#, err);

                            of_res.text_setup();

                            of_res.set_payload_from_str(&error_message);

                            if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                            {

                                return false;

                            }

                        }

                    }

                }
                OpCode::Binary =>
                {

                    let error_message = r#"{
                                                    "error": "The binary OpCode is not supported."
                                                }"#;

                    of_res.text_setup();

                    of_res.set_payload_from_str(error_message);

                    if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                    {

                        return false;

                    }

                    //To ResultProcessorActor... or not...

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

