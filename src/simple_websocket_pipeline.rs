use std::sync::Arc;

use fastwebsockets::upgrade::IncomingUpgrade;

use crate::Store;


pub struct SimpleWebSocketPipeline
{



}

impl SimpleWebSocketPipeline
{

    pub fn new(ws: IncomingUpgrade, store: Arc<Store>)
    {

        

    }

}