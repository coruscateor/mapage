use core::str;
use std::sync::Arc;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;
use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::{types::json::{Command, TypeInstance}, CommandResult, OwnedFrame, Store};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, EgressActorInput, ParsedInput};

use crate::types::json::SupportedType;

//Converts the parsed input into a command or a set of commands.

pub struct CommandExecutorActorState
{

    command_exector_reciver: Receiver<Command>,
    store: Arc<Store>,
    egress_actor_sender: Sender<EgressActorInput>

}

impl CommandExecutorActorState
{

    pub fn new(command_exector_reciver: Receiver<Command>, store: Arc<Store>, egress_actor_sender: Sender<EgressActorInput>) -> Self
    {

        Self
        {

            command_exector_reciver,
            store,
            egress_actor_sender

        }

    }

    pub fn spawn(store: Arc<Store>, egress_actor_sender: Sender<EgressActorInput>) -> Sender<Command>
    {

        let (sender, receiver) = channel(50);

        CommandExecutorActor::spawn(CommandExecutorActorState::new(receiver, store, egress_actor_sender));

        sender

    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(command) = self.command_exector_reciver.recv().await
        {

            self.execute_command(command).await;

            true

        }
        else
        {

            false
            
        }

    }

    async fn get_key_param(command: &Command) -> Result<&String, SendableText>
    {

        if let Some(params) = &command.params
        {

            if let Some(opt_key) = params.first()
            {

                if let Some(ti_key) = opt_key
                {

                    match ti_key
                    {

                        TypeInstance::String(key) =>
                        {

                            Ok(key)

                        }
                        _ =>
                        {

                            Err(SendableText::Str("The provided key parameter is the wrong type."))

                        }
                        
                    }

                }
                else
                {

                    Err(SendableText::Str("Key not provided."))
                    
                }

            }
            else
            {

                //Error: parameter list empty.
                
                Err(SendableText::Str("Provided parameter list empty."))

            }

        }
        else
        {

            Err(SendableText::Str("No parameter list list provided."))
            
        }

    }

    async fn execute_bool_command(&mut self, command: Command) -> Result<(), SendableText>
    {

        match command.command.as_str()
        {

            "get" =>
            {

                let key = Self::get_key_param(&command).await?;

                let res = self.store.bool_namespace().read(key).await;

                match res
                {

                    Ok(val) =>
                    {

                        let res = CommandResult
                        {

                            id: command.id,
                            result: TypeInstance::Bool(val),
                            message: None,
                            done: true

                        };
                        
                        Ok(())

                    }
                    Err(err) =>
                    {

                        Err(SendableText::String(err.to_string()))

                    }

                }

                /*
                if let Some(params) = &command.params
                {

                    if let Some(opt_key) = params.first()
                    {

                        if let Some(ti_key) = opt_key
                        {

                            match ti_key
                            {

                                TypeInstance::String(key) =>
                                {

                                    let res = self.store.bool_namespace().read(key).await;

                                    match res
                                    {

                                        Ok(val) =>
                                        {

                                            Ok(val)

                                        }
                                        Err(err) =>
                                        {

                                            Err(SendableText::String(err.to_string()))

                                        }

                                    }

                                }
                                _ =>
                                {

                                    Err(SendableText::Str("The provided key parameter is the wrong type."))

                                }
                                
                            }

                        }
                        else
                        {

                            Err(SendableText::Str("Key not provided."))
                            
                        }

                    }
                    else
                    {

                        //Error: parameter list empty.
                        
                        Err(SendableText::Str("Provided parameter list empty."))

                    }

                }
                else
                {

                    //Error: command has no parameters.

                    Err(SendableText::Str("No parameter list list provided."))
                    
                }
                */

            }
            "set" =>
            {

                Err(SendableText::Str("Nothng here"))

            }
            _ =>
            {

                Err(SendableText::Str("Nothng here"))

            }
            
        }

    }

    async fn execute_command(&mut self, command: Command) -> Result<(), SendableText>
    {

        if let Some(type_name) = command.type_name
        {

            match type_name
            {

                SupportedType::Bool => self.execute_bool_command(command).await,
                SupportedType::Char => todo!(),
                SupportedType::F32 => todo!(),
                SupportedType::F64 => todo!(),
                SupportedType::I8 => todo!(),
                SupportedType::I16 => todo!(),
                SupportedType::I32 => todo!(),
                SupportedType::I64 => todo!(),
                SupportedType::I128 => todo!(),
                SupportedType::Isize => todo!(),
                SupportedType::U8 => todo!(),
                SupportedType::U16 => todo!(),
                SupportedType::U32 => todo!(),
                SupportedType::U64 => todo!(),
                SupportedType::U128 => todo!(),
                SupportedType::Usize => todo!(),
                SupportedType::String => todo!(),
                SupportedType::Whatever => todo!(),
                SupportedType::VecBool => todo!(),
                SupportedType::VecChar => todo!(),
                SupportedType::VecF32 => todo!(),
                SupportedType::VecF64 => todo!(),
                SupportedType::VecI8 => todo!(),
                SupportedType::VecI16 => todo!(),
                SupportedType::VecI32 => todo!(),
                SupportedType::VecI64 => todo!(),
                SupportedType::VecI128 => todo!(),
                SupportedType::VecIsize => todo!(),
                SupportedType::VecU8 => todo!(),
                SupportedType::VecU16 => todo!(),
                SupportedType::VecU32 => todo!(),
                SupportedType::VecU64 => todo!(),
                SupportedType::VecU128 => todo!(),
                SupportedType::VecUsize => todo!(),
                SupportedType::VecString => todo!(),
                SupportedType::VecWhatever => todo!(),

            }

        }
        else
        {

            match command.command.as_str()
            {

                "features" =>
                {



                }
                "ser_for" =>
                {

                    

                }
                _ =>
                {



                }
                
            }

            Ok(())
            
        }

    }

}

impl_mac_task_actor!(CommandExecutorActor);
