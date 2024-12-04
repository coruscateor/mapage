use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;
use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::{Command, OwnedFrame};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, EgressActorInput};

use crate::types::json::into_command;

//JSON

pub type ParsedInput = Value;

//CommandProcessorActorState

//Converts the parsed input into a command or a set of commands or something else.

pub struct MessageProcessorActorState
{

    communication_processor_reciver: Receiver<ParsedInput>,
    command_executor_sender: Sender<Command>,
    egress_actor_input_sender: Sender<EgressActorInput>

}

impl MessageProcessorActorState
{

    pub fn new(communication_processor_reciver: Receiver<ParsedInput>, command_executor_sender: Sender<Command>, egress_actor_input_sender: &Sender<EgressActorInput>) -> Self
    {
        
        Self
        {

            communication_processor_reciver,
            command_executor_sender,
            egress_actor_input_sender: egress_actor_input_sender.clone()

        }
        
    }

    pub fn spawn(communication_executor_sender: Sender<Command>, egress_actor_input_sender: &Sender<EgressActorInput>) -> Sender<Value>
    {

        let (sender, receiver) = channel(50);

        MessageProcessorActor::spawn(MessageProcessorActorState::new(receiver, communication_executor_sender, egress_actor_input_sender));

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
                                            
                                            let error = EgressActorInput::Error(SendableText::String(err.to_string()));

                                            if let Err(_err) = self.egress_actor_input_sender.send(error).await
                                            {
                        
                                                return false;
                        
                                            }
                        
                                        }
                        
                                    }
    
                                }
                                _ =>
                                {
    
                                    let error = EgressActorInput::Error(SendableText::Str("Message type not recognised."));

                                    if let Err(_err) = self.egress_actor_input_sender.send(error).await
                                    {
                        
                                        return false;
                        
                                    }
                        
    
                                }
                            
                            }
    
                            break;
    
                        }
                        
                    }

                }
                _ => 
                {

                    let error = EgressActorInput::Error(SendableText::Str("Object provided must be a map."));

                    if let Err(_err) = self.egress_actor_input_sender.send(error).await
                    {

                        return false;

                    }

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

impl_mac_task_actor!(MessageProcessorActor);

