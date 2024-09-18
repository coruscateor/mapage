use std::sync::Arc;

use crossbeam::queue::ArrayQueue;
use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use fastwebsockets::{FragmentCollectorRead, Frame, OpCode, WebSocketError};
use tokio::select;

use crate::{OwnedFrame, Store};

use crate::WebSocketReader;

use crate::WebSocketWriteHalf;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor, impl_mac_task_actor_built_state};

use paste::paste;

use tokio::task::JoinHandle; //For impl_mac_task_actor_built_state

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

//Actor IO 

use super::array_queue::{ActorIOClient, ActorIOServer, actor_io};
use super::SimpleWebSocketActorInputMessage;

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

    //reader: WebSocketReader,
    writer: WebSocketWriteHalf,
    //reader_actor_recevier: Receiver<()>
    //reader_actor_sender: Sender<()>
    reader_actor_io_client: ActorIOClient<(), SimpleWebSocketActorInputMessage>
}

impl SimpleWebSocketActorState
{

    pub fn new(reader: WebSocketReader, writer: WebSocketWriteHalf) -> Self
    {

        //let arc_writer = Arc::new(writer); //.clone();

        //arc_writer.write_frame(frame)

        //The SimpleWebSocketReaderActor recever needs to be part of this object.

        let reader_actor_io_client = SimpleWebSocketReaderActorState::spawn(reader);

        Self
        {

            //reader,
            writer,
            //reader_actor_recevier
            //reader_actor_sender
            reader_actor_io_client
            
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
    //input_receiver: Receiver<()>
    io_server: ActorIOServer<(), SimpleWebSocketActorInputMessage>,
    obligated_send_frame_holder: Arc<ArrayQueue<OwnedFrame>>,
    
}

impl SimpleWebSocketReaderActorState
{

    pub fn new(reader: WebSocketReader) -> (Self, ActorIOClient<(), SimpleWebSocketActorInputMessage>) //, input_receiver: Receiver<()>, 
    {

        let (io_client, io_server) = actor_io(1, 1);

        (Self
        {

            reader,
            //input_receiver
            io_server,
            obligated_send_frame_holder: Arc::new(ArrayQueue::new(1))


        },
        io_client)

    }

    pub fn spawn(reader: WebSocketReader) -> ActorIOClient<(), SimpleWebSocketActorInputMessage>//Sender<()> //Receiver<()>
    {

        //let (input_sender, input_receiver) = channel(1);

        let (state, actor_io_client) = SimpleWebSocketReaderActorState::new(reader); //, input_receiver

        SimpleWebSocketReaderActor::spawn(state);

        actor_io_client

        //input_receiver

        //input_sender

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        //Connected here

        enum SelectResult<'f>
        {

            ReadFrame(Result<Frame<'f>, WebSocketError>),
            Connection(Option<()>)

        }

        let obligated_send_frame_holder = self.obligated_send_frame_holder.clone();

        let mut send_fn = |obligated_send_frame: Frame|
        {

            //Get from cache

            let mut of = OwnedFrame::new();

            of.copy_all_from_read_frame(&obligated_send_frame);

            let _ = obligated_send_frame_holder.push(of).expect("Error: The obligated_send_frame_holder should've been checked.");

            async
            {

                Result::<(), WebSocketError>::Ok(())

            }

        };

        loop
        {

            let should_exit = self.io_server.input_receiver().recv();

            let read_frame_future = self.reader.read_frame(&mut send_fn);

            let select_result;

            select!
            {

                biased;

                res = should_exit =>
                {

                    select_result = SelectResult::Connection(res);

                }
                res = read_frame_future =>
                {

                    select_result = SelectResult::ReadFrame(res);

                }

            }

            //Does an obligated frame need to be sent?

            //Copied from Req It.

            if let Some(mut of) = obligated_send_frame_holder.pop()
            {

                match of.opcode
                {
    
                    OpCode::Close =>
                    {

                        //Send close frame
    
                        of.clear_payload();

                        //From this message the WebSocketActor must either initiate the disconnection process or complete it.
                        
                        if let Err(_err) = self.io_server.output_sender().send(SimpleWebSocketActorInputMessage::WriteFrame(of)).await
                        {
        
                            return false;
        
                        }
                        
                    }
                    OpCode::Ping =>
                    {

                        //Now send a pong frame.

                        of.pong_setup();
                        
                        if let Err(_err) = self.io_server.output_sender().send(SimpleWebSocketActorInputMessage::WriteFrame(of)).await
                        {
        
                            return false;
        
                        }

                    }
                    OpCode::Continuation | OpCode::Text | OpCode::Binary | OpCode::Pong =>
                    {

                        //If for some reason you get another type of frame...

                        //Send next
                        
                        /*
                        let message = ReadFrameProcessorActorInputMessage::Frame(of);
    
                        let counted_message = self.pipeline_message_counter.increment_with_message_mut(message);
    
                        if let Err(_err) = self.read_frame_processor_actor_io_sender.send(counted_message).await
                        {

                            return false;

                        }
                        */
    
                    }
    
                }

            }

            match select_result
            {

                SelectResult::ReadFrame(frame_res) =>
                {

                    match frame_res
                    {

                        Ok(frame) =>
                        {

                            //Get from cache...

                            let mut of = OwnedFrame::new();

                            of.copy_all_from_read_frame(&frame);

                            //Send next

                        }
                        Err(err) =>
                        {

                            print!("{}", err);

                            break;

                        }

                    }


                }
                SelectResult::Connection(_input_opt) =>
                {

                    break;

                }

            }

        }

        false

    }


}

impl_mac_task_actor!(SimpleWebSocketReaderActor);

