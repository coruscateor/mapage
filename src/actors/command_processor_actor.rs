use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::OwnedFrame;

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::array_queue::ActorIOClient;

//JSON

pub type ParsedInput = Value;

//Converts the parsed input into a command or a set of commands.

pub struct CommandProcessorActorState
{

    command_processor_reciver: Receiver<ParsedInput>

}

impl CommandProcessorActorState
{

    pub fn new(command_processor_reciver: Receiver<ParsedInput>, /* Egress Sender */) -> Self
    {

        Self
        {

            command_processor_reciver

        }

    }

    pub fn spawn() //-> Receiver<()>
    {



    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(parsed_input) = self.command_processor_reciver.recv().await
        {

            

        }
        else
        {

            return false;
            
        }

        true

    }

}

