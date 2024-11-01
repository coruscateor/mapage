use std::process::Command;
use std::sync::Arc;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use crate::Store;

use crate::actors::{CommandExecutorActorState, CommunicationProcessorActorState, IngressActorState, SimpleWebSocketActorState};

pub struct SimpleWebSocketPipeline
{



}

impl SimpleWebSocketPipeline
{

    pub fn new(fut: UpgradeFut, store: Arc<Store>)
    {

        let simple_websoicket_actor_io_client = SimpleWebSocketActorState::spawn(fut);

        //simple_websoicket_actor_io_client.input_sender().re

        //Check if upgrade is successful at some point.

        let cea_sender = CommandExecutorActorState::spawn(store);

        let cpa_sender = CommunicationProcessorActorState::spawn(cea_sender);

        IngressActorState::spawn(&simple_websoicket_actor_io_client, cpa_sender);

    }

}