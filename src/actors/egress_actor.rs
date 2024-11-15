use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};
use serde::{ser::{SerializeMap, SerializeStruct}, Serialize};

use crate::{CommandError, CommandResult, OwnedFrame};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, ParsedInput};

pub enum EgressActorInput
{

    Error(SendableText),
    CommandError(CommandError),
    CommandResult(CommandResult)

}

/*
struct ErrorHelperStruct
{

    error: SendableText

}

impl Serialize for ErrorHelperStruct
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {

        todo!()

    }

}

struct CommandErrorHelperStruct
{

    command_error: CommandError

}

impl Serialize for CommandErrorHelperStruct
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {

        todo!()

    }

}

struct CommandResultHelperStruct
{

    command_error: CommandError

}

impl Serialize for CommandResultHelperStruct
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer
    {

        todo!()

    }

}

impl Serialize for EgressActorInput
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {

        match self
        {

            EgressActorInput::Error(sendable_text) =>
            {

                //let mut error = serializer.serialize_struct("Error", 1)?;

                //error.serialize_field("error", sendable_text)?;

                //let error_struct = error.end()?;

                //let mut interior = serializer.serialize_map(Some(1))?;

                //interior.serialize_entry("error", sendable_text);

                //let interior_entry = interior.end()?;

                let mut map= serializer.serialize_map(Some(1))?;

                map.serialize_entry("Error",&error_struct)?; //&interior_entry)?;

                map.end()

            }
            EgressActorInput::CommandError(command_error) =>
            {

                let mut interior = serializer.serialize_map(Some(1))?;

                interior.serialize_entry("Error", command_error);

                let interior_entry = interior.end()?;

                let mut map= serializer.serialize_map(Some(1))?;

                map.serialize_entry("command_error", &interior_entry.into());

                map.end()

            }
            EgressActorInput::CommandResult(command_result) =>
            {

                let mut interior = serializer.serialize_map(Some(1))?;

                interior.serialize_entry("Result", command_result);

                let interior_entry = interior.end()?;

                let mut map= serializer.serialize_map(Some(1))?;

                map.serialize_entry("command_result", &interior_entry.into());

                map.end()

            }
            
        }

    }

}
*/

//Converts results and errors into OwnedFrames.

pub struct EgressActorState
{

    websocket_actor_input_sender: Sender<OwnedFrame>,
    //command_processor_sender: Sender<ParsedInput>
    reciver: Receiver<EgressActorInput>

}

impl EgressActorState
{

    pub fn new(websocket_actor_input_sender: &Sender<OwnedFrame>, reciver: Receiver<EgressActorInput>) -> Self
    {

        Self
        {

            websocket_actor_input_sender: websocket_actor_input_sender.clone(),
            //command_processor_sender
            reciver

        }

    }

    pub fn spawn(websocket_actor_input_sender: &Sender<OwnedFrame>) -> Sender<EgressActorInput>
    {

        let (sender, reciver) = channel(50);

        EgressActor::spawn(EgressActorState::new(websocket_actor_input_sender, reciver));

        sender

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        if let Some(input) = self.reciver.recv().await
        {

            self.evaluate_input(input).await

            //true

        }
        else
        {

            false

        }

    }

    //JSON

    async fn evaluate_input(&mut self, input: EgressActorInput) -> bool
    {

        match input
        {

            EgressActorInput::Error(sendable_text) =>
            {

                let value = json!({

                    "error": {
                        "error:": sendable_text
                    }

                });

                let json_to_send = value.to_string();

                self.send(&json_to_send).await

                //Get from cache
                
                //let mut of = OwnedFrame::new();

                //of.text_setup();
                
                //of.set_payload_from_str(&json_to_send);

            }
            EgressActorInput::CommandError(command_error) =>
            {

                let value = json!({

                    "error": {
                        "command_error:": command_error
                    }

                });

                let json_to_send = value.to_string();

                self.send(&json_to_send).await

            },
            EgressActorInput::CommandResult(command_result) =>
            {

                let value = json!({

                    "result": {
                        "command_result:": command_result
                    }

                });

                let json_to_send = value.to_string();

                self.send(&json_to_send).await
                
            }

        }

    }

    //Divide the message into 50mb frames and send each.

    async fn send(&mut self, text: &String) -> bool
    {

        let text_bytes = text.as_bytes();

        //52428800 is the number of bytes in 50 megabytes.

        let mut ce = text_bytes.chunks_exact(52428800);

        let mut is_multi_part = false;

        if let Some(first) = ce.next()
        {

            is_multi_part = true;

            //Get from cache

            let mut of = OwnedFrame::new();

            //Begin contuniation

            of.text_continuation_stater_setup();

            //Send the first chunk.

            of.set_payload_from_bytes(first);

            if let Err(_err) = self.websocket_actor_input_sender.send(of).await
            {

               return false;

            }

            //for item in ce

            loop
            {

                if let Some(item) = ce.next()
                {

                    //Get from cache
        
                    let mut of = OwnedFrame::new();
        
                    of.continuation_setup();
                    
                    of.set_payload_from_bytes(item);

                    if let Err(_err) = self.websocket_actor_input_sender.send(of).await
                    {
        
                        return false;
        
                    }

                }
                else
                {

                    break;
                    
                }
    
            }

        }

        let remainder = ce.remainder();

        if remainder.len() > 0
        {
            //Get from cache

            let mut of = OwnedFrame::new();
            
            if is_multi_part
            {

                 of.final_continuation_setup();

                 of.set_payload_from_bytes(remainder);

            }
            else
            {

                of.text_setup();

                of.set_payload_from_bytes(remainder);

            }

            if let Err(_err) = self.websocket_actor_input_sender.send(of).await
            {

               return false;

            }

        }

        true

    }

}

impl_mac_task_actor!(EgressActor);