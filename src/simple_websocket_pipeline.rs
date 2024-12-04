use std::sync::Arc;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use crate::Store;

use crate::actors::{CommandExecutorActorState, EgressActorState, IngressActorState, SimpleWebSocketActorState}; //MessageProcessorActorState, 

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

        let ea_sender = EgressActorState::spawn(simple_websoicket_actor_io_client.input_sender());

        let cea_sender = CommandExecutorActorState::spawn(store, &ea_sender);

        //let cpa_sender = MessageProcessorActorState::spawn(cea_sender, &ea_sender);

        IngressActorState::spawn(simple_websoicket_actor_io_client.output_receiver(),  cea_sender, &ea_sender); //cpa_sender,

    }

}