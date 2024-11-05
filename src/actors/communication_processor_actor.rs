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

//CommandProcessorActorState

//Converts the parsed input into a command or a set of commands or something else.

pub struct CommunicationProcessorActorState
{

    communication_processor_reciver: Receiver<ParsedInput>,
    command_executor_sender: Sender<Command>

}

impl CommunicationProcessorActorState
{

    pub fn new(communication_processor_reciver: Receiver<ParsedInput>, command_executor_sender: Sender<Command> /* Egress Sender */) -> Self
    {

        Self
        {

            communication_processor_reciver,
            command_executor_sender

        }

    }

    pub fn spawn(communication_executor_sender: Sender<Command> /* Egress Sender */) -> Sender<Value>
    {

        let (sender, receiver) = channel(50);

        CommunicationProcessorActor::spawn(CommunicationProcessorActorState::new(receiver, communication_executor_sender));

        sender

    }

    impl_default_start_and_end_async!();
    
    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(parsed_input) = self.communication_processor_reciver.recv().await
        {

            //What type of communication object is it?

            match parsed_input
            {

                Value::Object(map) =>
                {

                    if map.is_empty()
                    {

                        //Error

                    }
                    else
                    {

                        for kvp in map
                        {
    
                            match kvp.0.as_str()
                            {
    
                                "command" =>
                                {
    
                                    match into_command(kvp.1)
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
                                _ =>
                                {
    
                                    //Error
    
                                }
                            
                            }
    
                            break;
    
                        }
                        
                    }

                }
                _ => 
                {

                    //Error

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

impl_mac_task_actor!(CommunicationProcessorActor);

