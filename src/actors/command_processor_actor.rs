use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::{types::json::Command, OwnedFrame};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::array_queue::ActorIOClient;

use crate::types::json::into_command;

//JSON

pub type ParsedInput = Value;

//Converts the parsed input into a command or a set of commands.

pub struct CommandProcessorActorState
{

    command_processor_reciver: Receiver<ParsedInput>,
    command_executor_sender: Sender<Command>

}

impl CommandProcessorActorState
{

    pub fn new(command_processor_reciver: Receiver<ParsedInput>, command_executor_sender: Sender<Command> /* Egress Sender */) -> Self
    {

        Self
        {

            command_processor_reciver,
            command_executor_sender

        }

    }

    pub fn spawn(command_executor_sender: Sender<Command> /* Egress Sender */) -> Sender<Value>
    {

        let (sender, receiver) = channel(50);

        CommandProcessorActor::spawn(CommandProcessorActorState::new(receiver, command_executor_sender));

        sender

    }

    impl_default_start_and_end_async!();
    
    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(parsed_input) = self.command_processor_reciver.recv().await
        {

            match into_command(parsed_input)
            {

                Ok(res) =>
                {

                    if let Err(_err) = self.command_executor_sender.send(res).await
                    {

                        return false;

                    }

                }
                Err(err) =>
                {

                    //To EgressActor

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

impl_mac_task_actor!(CommandProcessorActor);

