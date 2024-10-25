use core::str;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor};

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use crate::OwnedFrame;

use paste::paste;

use tokio::task::JoinHandle; 

use fastwebsockets::OpCode;

use serde_json::{from_str, json, Value};

use super::{array_queue::ActorIOClient, ParsedInput};

//Converts results and errors into OwnedFrames.

pub struct EgressActorState
{

    websocket_actor_input_sender: Sender<OwnedFrame>,
    //command_processor_sender: Sender<ParsedInput>
    //egress_actor_sender

}

impl EgressActorState
{

    pub fn new(websocket_actor_input_sender: &Sender<OwnedFrame> /*, command_processor_sender: Sender<ParsedInput>*/) -> Self //actor_io_reciver: Receiver<OwnedFrame>) -> Self
    {

        Self
        {

            websocket_actor_input_sender: websocket_actor_input_sender.clone(),
            //command_processor_sender

        }

    }

    pub fn spawn(websocket_actor_input_sender: &Sender<OwnedFrame>) //-> Receiver<()>
    {

        EgressActor::spawn(EgressActorState::new(websocket_actor_input_sender));

    }

    impl_default_start_and_end_async!();

    //JSON

    async fn run_async(&mut self) -> bool
    {

        false

    }

}

impl_mac_task_actor!(EgressActor);