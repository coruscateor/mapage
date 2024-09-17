use std::sync::Arc;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use fastwebsockets::FragmentCollectorRead;

use crate::Store;

use crate::WebSocketReader;

use crate::WebSocketWriteHalf;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor, impl_mac_task_actor_built_state};

use paste::paste;

use tokio::task::JoinHandle; //For impl_mac_task_actor_built_state

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

pub struct SimpleWebSocketActorStateBuilder
{

    upgrade_fut: UpgradeFut

}

impl SimpleWebSocketActorStateBuilder
{

    pub async fn build_async(self) -> Option<SimpleWebSocketActorState>
    {

        match self.upgrade_fut.await
        {

            Ok(res) =>
            {

                let (read, write) = res.split(tokio::io::split);

                let reader = WebSocketReader::FragmentCollectorRead(FragmentCollectorRead::new(read));

                Some(SimpleWebSocketActorState::new(reader, write))

            }
            Err(err) =>
            {

                println!("{}", err);

                None

            }

        }

    }

}

//Write Side

pub struct SimpleWebSocketActorState
{

    reader: WebSocketReader,
    writer: WebSocketWriteHalf

}

impl SimpleWebSocketActorState
{

    pub fn new(reader: WebSocketReader, writer: WebSocketWriteHalf) -> Self
    {

        Self
        {

            reader,
            writer

        }

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        //Get frame from previous stage



        true

    }

}

impl_mac_task_actor_built_state!(SimpleWebSocketActor);

//Read Side

pub struct SimpleWebSocketReaderActorState
{

    reader: WebSocketReader,
    input_sender: Sender<()>

}

impl SimpleWebSocketReaderActorState
{

    pub fn new(reader: WebSocketReader, input_sender: Sender<()>, ) -> Self
    {

        Self
        {

            reader,
            input_sender


        }

    }

    pub fn spawn(reader: WebSocketReader) -> Receiver<()>
    {

        let (input_sender, input_receiver) = channel(1);

        SimpleWebSocketReaderActor::spawn(SimpleWebSocketReaderActorState::new(reader, input_sender));

        input_receiver

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        //Get frame from previous stage



        true

    }


}

impl_mac_task_actor!(SimpleWebSocketReaderActor);

