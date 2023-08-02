
use act_rusty::*;
use async_trait::async_trait;

use super::actor_message::*;

pub struct StorageActorIO //<'a> //IO
{

    qp: NotifyingArrayQueuePusher<ActorMessage> //<'a>>

}

impl /*<'a>*/ StorageActorIO //<'a>
{

    pub fn new(qp: NotifyingArrayQueuePusher<ActorMessage>) -> Self
    {

        Self
        {

            qp

        }

    }

    pub fn get_notifying_array_queue_pusher(&self) -> &NotifyingArrayQueuePusher<ActorMessage> //<'a>>
    {

        &self.qp

    }

}

impl /*<'a>*/ ActorIO for StorageActorIO //<'a>
{

    fn send_default(&self)
    {
    
        self.qp.push_notify_one(ActorMessage::None);
        
    }

}
