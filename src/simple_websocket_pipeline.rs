use std::sync::Arc;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use crate::Store;

use crate::actors::SimpleWebSocketActorState;

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

        

    }

}