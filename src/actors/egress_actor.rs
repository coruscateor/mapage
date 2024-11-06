use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};
use serde::{ser::SerializeMap, Serialize};

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

                let mut interior = serializer.serialize_map(Some(1))?;

                interior.serialize_entry("error", sendable_text);

                let interior_entry = interior.end()?;

                let mut map= serializer.serialize_map(Some(1))?;

                map.serialize_entry("Error", &interior_entry.into());

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

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(input) = self.reciver.recv().await
        {


            true

        }
        else
        {

            false

        }

    }

}

impl_mac_task_actor!(EgressActor);