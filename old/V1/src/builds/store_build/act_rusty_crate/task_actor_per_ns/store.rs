use std::collections::hash_map::RandomState;

use scc::HashMap;

use super::namespace::Namespace;

use act_rusty::*;

use crate::{stored_object::StoredObject, all_types::Rcd};

use async_trait::async_trait;

use crate::storage_container::*;

use crate::actor_utils::*;

pub struct Store
{

    default: Namespace,
    namespaces: HashMap<String, Namespace> //Rcd<dyn StorageContainer>> //TS_IO_Actor<Nsc, >> //Actor_IO>> //KeyValueStore>
    //namespaces: HashMap<String, Box<dyn TS_IO_Actor<AsyncActorStateController<ActorIO>, ActorIO>>>

}

impl Store
{

    pub fn new() -> Self
    {

        Self
        {

            default: Namespace::new(),
            namespaces: HashMap::new(1000, RandomState::new())

        }

    }

}

/*
struct Actor_IO //IO
{

}

impl ActorIO for Actor_IO
{
    fn send_default(&self) {
        todo!()
    }
}

struct Nsc //State Controller
{

    value: Box<dyn StoredObject>

}

impl Nsc
{

    pub fn new(value: Box<dyn StoredObject>) -> Self
    {

        Self
        {

            value

        }

    }

}

#[async_trait]
impl AsyncActorStateController<Actor_IO> for Nsc
{
    fn get_IO(&self) -> std::sync::Arc<Actor_IO>  {
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
*/

