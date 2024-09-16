use std::sync::Arc;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use crate::Store;


pub struct SimpleWebSocketActorStateBuilder
{

    upgrade_fut: UpgradeFut

}

impl SimpleWebSocketActorStateBuilder
{

    pub async fn build_async(self) -> (bool, SimpleWebSocketActorState)
    {

        match self.upgrade_fut.await
        {

            Ok(res) =>
            {

                

            }
            Err(err) =>
            {

                println!("{}", err);

                None

            }

        }

    }

}

pub struct SimpleWebSocketActorState
{



}

impl SimpleWebSocketActorState
{

    pub fn new(incoming_upgrade: IncomingUpgrade)
    {

        

    }

}

