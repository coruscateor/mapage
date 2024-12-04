use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;
use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};
use serde::Serialize;

use crate::{Command, OwnedFrame};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, EgressActorInput}; //, ParsedInput};

use crate::StreamedMessage;

//Converts the OwnedFrames payload into an in-memory representation of the desired format and passes it on. 

pub struct IngressActorState
{

    //websocket_actor_io_client: ActorIOClient<OwnedFrame, OwnedFrame>,
    websocket_actor_output_receiver: Receiver<OwnedFrame>,
    //command_processor_sender: Sender<ParsedInput>,
    command_exeutor_sender: Sender<Command>,
    egress_actor_input_sender: Sender<EgressActorInput>

}

impl IngressActorState
{

    pub fn new(websocket_actor_output_receiver: &Receiver<OwnedFrame>, command_exeutor_sender: Sender<Command>, egress_actor_input_sender: &Sender<EgressActorInput>) -> Self //websocket_actor_io_client: &ActorIOClient<OwnedFrame, OwnedFrame>, //actor_io_reciver: Receiver<OwnedFrame>) -> Self
    {

        Self
        {

            //websocket_actor_io_client: websocket_actor_io_client.clone(),
            websocket_actor_output_receiver: websocket_actor_output_receiver.clone(),
            command_exeutor_sender,
            egress_actor_input_sender: egress_actor_input_sender.clone()

        }

    }

    pub fn spawn(websocket_actor_output_receiver: &Receiver<OwnedFrame>, command_executor_sender: Sender<Command>, egress_actor_input_sender: &Sender<EgressActorInput>) //command_processor_sender: Sender<ParsedInput>, //websocket_actor_io_client: &ActorIOClient<OwnedFrame, OwnedFrame>, //-> Receiver<()>
    {

        IngressActor::spawn( IngressActorState::new(websocket_actor_output_receiver, command_executor_sender, egress_actor_input_sender)); //websocket_actor_io_client, 

    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(of_res) = self.websocket_actor_output_receiver.recv().await //self.websocket_actor_io_client.output_receiver().recv().await //actor_io_reciver.recv().await
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

                    //Cache of_res here

                    match con_res
                    {

                        Ok(res) =>
                        {

                            let message_res = from_str::<'_, StreamedMessage>(res);

                            match message_res
                            {

                                Ok(res) =>
                                {

                                    match res
                                    {

                                        StreamedMessage::Command(command) =>
                                        {

                                            if let Err(_err) = self.command_exeutor_sender.send(command).await
                                            {
        
                                                return false;
                
                                            }

                                        }
                                        StreamedMessage::CommandResult(command_result) =>
                                        {

                                            print_unexpected_input(command_result);

                                        }
                                        StreamedMessage::CommandError(command_error) =>
                                        {

                                            print_unexpected_input(command_error);

                                        }
                                        StreamedMessage::Error(sendable_text) =>
                                        {

                                            print_unexpected_input(sendable_text);

                                        }

                                    }

                                }
                                Err(err) =>
                                {

                                    let err_message = EgressActorInput::Error(err.to_string().into());

                                    if let Err(_err) = self.egress_actor_input_sender.send(err_message).await
                                    {
        
                                        return false;
        
                                    }

                                }

                            }

                            /*
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

                                    let err_message = EgressActorInput::Error(SendableText::String(err.to_string()));

                                    if let Err(_err) = self.egress_actor_input_sender.send(err_message).await
                                    {

                                        return false;

                                    }

                                    /*
                                    let error_message = format!(r#"
                                       {{ "error": "{}" }}
                                    "#, err);

                                    of_res.text_setup();

                                    of_res.set_payload_from_str(&error_message);

                                    if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                                    {

                                        return false;

                                    }
                                    */

                                }

                            }
                            */

                        }
                        Err(err) =>
                        {

                            let err_message = EgressActorInput::Error(SendableText::String(err.to_string()));

                            if let Err(_err) = self.egress_actor_input_sender.send(err_message).await
                            {

                                return false;

                            }

                            /*
                            let error_message = format!(r#"
                                {{ "error": "{}" }}
                            "#, err);

                            of_res.text_setup();

                            of_res.set_payload_from_str(&error_message);

                            if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                            {

                                return false;

                            }
                            */
                        }

                    }

                }
                OpCode::Binary =>
                {

                    let err_message = EgressActorInput::Error(SendableText::Str("The binary OpCode is not supported."));

                    if let Err(_err) = self.egress_actor_input_sender.send(err_message).await
                    {

                        return false;

                    }

                    /*
                    let error_message = r#"{
                                                    "error": "The binary OpCode is not supported."
                                                }"#;

                    of_res.text_setup();

                    of_res.set_payload_from_str(error_message);

                    if let Err(_err) = self.websocket_actor_io_client.input_sender().send(of_res).await
                    {

                        return false;

                    }
                    */

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



fn print_unexpected_input<T>(input: T)
    where T: Serialize
{

let to_json_res = serde_json::to_string(&input);

let input;

match to_json_res
{

    Ok(res) =>
    {

        input = res;

    }
    Err(err) =>
    {

        input = err.to_string();

    }

}

println!("Unexpected input:\n\n {}", input);

}

