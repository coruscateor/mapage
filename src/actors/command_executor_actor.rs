use core::str;
use std::sync::Arc;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use corlib::text::SendableText;

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::{Command, types::TypeInstance, CommandError, CommandResult, OwnedFrame, Store};

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, EgressActorInput}; //, ParsedInput};

use crate::types::SupportedType;

//use crate::command_execution::ExecutionResult;

use crate::command_execution::{CommandExecutor, ExecutionResult};

//type ExecutionResult = Result<CommandResult, CommandError>;

//Executes provided commands on the store.

pub struct CommandExecutorActorState
{

    command_exector_reciver: Receiver<Command>,
    //store: Arc<Store>,
    egress_actor_input_sender: Sender<EgressActorInput>,
    command_executor: CommandExecutor

}

impl CommandExecutorActorState
{

    pub fn new(command_exector_reciver: Receiver<Command>, store: Arc<Store>, egress_actor_input_sender: &Sender<EgressActorInput>) -> Self
    {

        Self
        {

            command_exector_reciver,
            //store,
            egress_actor_input_sender: egress_actor_input_sender.clone(),
            command_executor: CommandExecutor::new(store)

        }

    }

    pub fn spawn(store: Arc<Store>, egress_actor_input_sender: &Sender<EgressActorInput>) -> Sender<Command>
    {

        let (sender, receiver) = channel(50);

        CommandExecutorActor::spawn(CommandExecutorActorState::new(receiver, store, egress_actor_input_sender));

        sender

    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        if let Some(command) = self.command_exector_reciver.recv().await
        {

            let execution_result = self.command_executor.execute_command(command).await;

            match execution_result
            {

                Ok(res) =>
                {

                    let _ = self.egress_actor_input_sender.send(EgressActorInput::CommandResult(res));

                }
                Err(err) =>
                {

                    let _ = self.egress_actor_input_sender.send(EgressActorInput::CommandError(err));

                }

            }

            true

        }
        else
        {

            false
            
        }

    }

}

impl_mac_task_actor!(CommandExecutorActor);
