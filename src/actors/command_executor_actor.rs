use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::{types::json::Command, OwnedFrame};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, ParsedInput};

use crate::types::json::SupportedType;

//Converts the parsed input into a command or a set of commands.

pub struct CommandExecutorActorState
{

    command_exector_reciver: Receiver<Command>

}

impl CommandExecutorActorState
{

    pub fn new(command_exector_reciver: Receiver<Command>, /* Egress Sender */) -> Self
    {

        Self
        {

            command_exector_reciver

        }

    }

    pub fn spawn() -> Sender<Command>
    {

        let (sender, receiver) = channel(50);

        CommandExecutorActor::spawn(CommandExecutorActorState::new(receiver));

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

    async fn execute_bool_command(&mut self, command: Command)
    {

        match command.command.as_str()
        {

            "get" =>
            {



            }
            "set" =>
            {



            }
            _ =>
            {



            }
            
        }

    }

    async fn execute_command(&mut self, command: Command)
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
            
        }

    }

}

impl_mac_task_actor!(CommandExecutorActor);
