use std::collections::HashMap;

use act_rusty::*;
use async_trait::async_trait;

use crate::{stored_object::*, storage_container::StorageContainer};

use crate::actor_utils::*;

pub struct Namespace
{

    //kvs: HashMap<String, Box<dyn StorageContainer>>
    actor: TS_IO_Actor<NamespaceActorController, StorageActorIO>

}

impl Namespace
{

    pub fn new() -> Self
    {

        Self
        {

            //kvs: HashMap::new()
            actor: TS_IO_Actor::new(NamespaceActorController::new())

        }

    }

}

struct NamespaceActorController //State Controller
{

    //value: Box<dyn StoredObject>
    kvs: HashMap<String, Box<dyn StoredObject>>

}

impl NamespaceActorController
{

    

    pub fn new() -> Self
    {

        Self
        {

            kvs: HashMap::new()

        }

    }

}

#[async_trait]
impl AsyncActorStateController<StorageActorIO> for NamespaceActorController
{
    fn get_IO(&self) -> std::sync::Arc<StorageActorIO>  {
        todo!()
    }

    fn on_enter_async< 'life0, 'async_trait>(& 'life0 mut self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = ()> + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    fn run_async< 'life0, 'life1, 'async_trait>(& 'life0 mut self,di: & 'life1 DroppedIndicator) ->  core::pin::Pin<Box<dyn core::future::Future<Output = bool> + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait, 'life1: 'async_trait,Self: 'async_trait {
        todo!()
    }

    fn on_exit_async< 'life0, 'async_trait>(& 'life0 mut self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = ()> + core::marker::Send+ 'async_trait> >where 'life0: 'async_trait,Self: 'async_trait {
        todo!()
    }

    fn get_which_paniced_handler(&self) -> WhichPanicedHandler {
        todo!()
    }
} 