use std::sync::Arc;

use libsync::ReceiveResult;
use tokio::time::{Duration, Instant};

use crossbeam::queue::ArrayQueue;

use fastwebsockets::upgrade::{IncomingUpgrade, UpgradeFut};

use fastwebsockets::{FragmentCollectorRead, Frame, OpCode, WebSocketError};

use tokio::select;

use crate::{owned_frame, OwnedFrame, Store};

use crate::WebSocketReader;

use crate::WebSocketWriteHalf;

use act_rs::{impl_default_end_async, impl_default_start_and_end_async, impl_default_start_async, impl_mac_task_actor, impl_mac_task_actor_built_state};

use paste::paste;

use tokio::task::JoinHandle; //For impl_mac_task_actor_built_state

use libsync::crossbeam::mpmc::tokio::array_queue::{Sender, Receiver, channel};

use tokio::time::timeout_at;

//Actor IO 

use super::array_queue::{ActorIOClient, ActorIOServer, actor_io};

use super::SimpleWebSocketActorInputMessage;

pub struct SimpleWebSocketActorStateBuilder
{

    upgrade_fut: UpgradeFut,
    actor_io_server: ActorIOServer<OwnedFrame, OwnedFrame>

}

impl SimpleWebSocketActorStateBuilder
{

    pub fn new(upgrade_fut: UpgradeFut, actor_io_server: ActorIOServer<OwnedFrame, OwnedFrame>) -> Self
    {

        Self
        {

            upgrade_fut,
            actor_io_server

        }

    }

    pub async fn build_async(self) -> Option<SimpleWebSocketActorState>
    {

        match self.upgrade_fut.await
        {

            Ok(res) =>
            {

                //Build the SimpleWebSocketActorState

                let (read, write) = res.split(tokio::io::split);

                let reader = WebSocketReader::FragmentCollectorRead(FragmentCollectorRead::new(read));

                Some(SimpleWebSocketActorState::new(reader, write, self.actor_io_server))

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
    reader_actor_io_client: ActorIOClient<(), SimpleWebSocketActorInputMessage>,
    //actor_io_server: ActorIOServer<OwnedFrame, OwnedFrame>
    actor_io_receiver: Receiver<OwnedFrame> //External input

}

impl SimpleWebSocketActorState
{

    pub fn new(reader: WebSocketReader, writer: WebSocketWriteHalf, actor_io_server: ActorIOServer<OwnedFrame, OwnedFrame>) -> Self //(Self, ActorIOClient<OwnedFrame, OwnedFrame>)
    {

        //let arc_writer = Arc::new(writer); //.clone();

        //arc_writer.write_frame(frame)

        //The SimpleWebSocketReaderActor recever needs to be part of this object.

        //let (io_client, io_server) = actor_io(50, 50);

        let reader_actor_io_client = SimpleWebSocketReaderActorState::spawn(reader, actor_io_server.output_sender().clone());

        Self
        {

            //reader,
            writer,
            //reader_actor_recevier
            //reader_actor_sender
            reader_actor_io_client,
            actor_io_receiver: actor_io_server.input_receiver().clone()
            
        } //,
        //io_client)

    }

    pub fn spawn(upgrade_fut: UpgradeFut) -> ActorIOClient<OwnedFrame, OwnedFrame>
    {

        let (io_client, io_server) = actor_io(50, 50);

        let state_builder = SimpleWebSocketActorStateBuilder::new(upgrade_fut, io_server);

        SimpleWebSocketActor::spawn(state_builder);

        io_client

    }

    impl_default_start_and_end_async!();

    async fn run_async(&mut self) -> bool
    {

        //Get frame from previous stage

        enum SelectResult
        {

            WriteFrame(Option<OwnedFrame>),
            FromSimpleWebSocketReaderActor(Option<SimpleWebSocketActorInputMessage>)

        }

        let reader_actor_io_client_recv = self.reader_actor_io_client.output_receiver().recv();

        let write_frame_future = self.actor_io_receiver.recv();

        let select_result;

        select!
        {

            biased;

            res = reader_actor_io_client_recv =>
            {

                select_result = SelectResult::FromSimpleWebSocketReaderActor(res);

            }
            res = write_frame_future =>
            {

                select_result = SelectResult::WriteFrame(res);

            }

        }

        match select_result
        {

            SelectResult::WriteFrame(opt_owned_frame) =>
            {

                if let Some(mut owned_frame) = opt_owned_frame
                {

                    if owned_frame.opcode == OpCode::Close
                    {

                        //Re-evaluate how this is done.

                        let now = Instant::now();

                        let soon = now.checked_add(Duration::from_secs(10)).expect("Error: Instant problems");            

                        let reader_actor_io_client_recv = self.reader_actor_io_client.output_receiver().recv();

                        match timeout_at(soon, reader_actor_io_client_recv).await
                        {

                            Ok(opt_res) =>
                            {

                                if let Some(res) = opt_res
                                {

                                    match res
                                    {

                                        SimpleWebSocketActorInputMessage::Disconnect => {}
                                        SimpleWebSocketActorInputMessage::WriteFrame(mut owned_frame) =>
                                        {

                                            let mut opcode = owned_frame.opcode;

                                            let frame = owned_frame.new_frame_to_be_written();

                                            if let Err(err) = self.writer.write_frame(frame).await
                                            {

                                                print!("{}", err);

                                            }

                                            loop
                                            {

                                                if opcode != OpCode::Close
                                                {

                                                    //If you don't have the close frame yet, find the close frame with the OwnedFrames you have available.

                                                    if let ReceiveResult::Ok(message) = self.reader_actor_io_client.output_receiver().try_recv()
                                                    {

                                                        match message
                                                        {

                                                            SimpleWebSocketActorInputMessage::Disconnect =>
                                                            { 
                                                                
                                                                break;

                                                            }
                                                            SimpleWebSocketActorInputMessage::WriteFrame(mut owned_frame) =>
                                                            {

                                                                let frame = owned_frame.new_frame_to_be_written();

                                                                if let Err(err) = self.writer.write_frame(frame).await
                                                                {
            
                                                                    print!("{}", err);

                                                                    break;
            
                                                                }

                                                                opcode = owned_frame.opcode;

                                                                //Cache OwnedFrame

                                                                /*
                                                                if opcode == OpCode::Close
                                                                {

                                                                    //Close OpCode received now exit.

                                                                    break;

                                                                }
                                                                */

                                                            }

                                                        }

                                                    }
    
                                                }
                                                else
                                                {
                                                    
                                                    break;

                                                }

                                            }
                                            
                                        }

                                    }

                                }

                            }
                            Err(err) =>
                            {

                                print!("{}", err);

                            }

                        }

                        return false;

                    }

                    let frame = owned_frame.new_frame_to_be_written();

                    if let Err(err) = self.writer.write_frame(frame).await
                    {

                        print!("{}", err);

                    }
                    else
                    {

                        //Cache OwnedFrame

                        return true;
                        
                    }

                }

            }
            SelectResult::FromSimpleWebSocketReaderActor(opt_simple_web_socket_actor_input_message) =>
            {

                if let Some(input_message) = opt_simple_web_socket_actor_input_message
                {

                    match input_message
                    {

                        SimpleWebSocketActorInputMessage::Disconnect => {}
                        SimpleWebSocketActorInputMessage::WriteFrame(mut owned_frame) =>
                        {

                            let frame = owned_frame.new_frame_to_be_written();

                            if let Err(err) = self.writer.write_frame(frame).await
                            {
        
                                print!("{}", err);
        
                            }
                            else
                            {

                                let opcode = owned_frame.opcode;

                                //Cache OwnedFrame

                                if opcode == OpCode::Close
                                {

                                    //Close frames have been sent and received.

                                    let _ = self.reader_actor_io_client.input_sender().send(());

                                    return false;

                                }
        
                                return true;
                                
                            }

                        }

                    }

                }

            }

        }

        false

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
    actor_io_sender: Sender<OwnedFrame> //External, from SimpleWebSocketActorState, output.
    
}

impl SimpleWebSocketReaderActorState
{

    pub fn new(reader: WebSocketReader, actor_io_sender: Sender<OwnedFrame>) -> (Self, ActorIOClient<(), SimpleWebSocketActorInputMessage>) //, input_receiver: Receiver<()>, 
    {

        let (io_client, io_server) = actor_io(1, 1);

        (Self
        {

            reader,
            //input_receiver
            io_server,
            obligated_send_frame_holder: Arc::new(ArrayQueue::new(1)),
            actor_io_sender


        },
        io_client)

    }

    pub fn spawn(reader: WebSocketReader, actor_io_sender: Sender<OwnedFrame>) -> ActorIOClient<(), SimpleWebSocketActorInputMessage>//Sender<()> //Receiver<()>
    {

        //let (input_sender, input_receiver) = channel(1);

        let (state, actor_io_client) = SimpleWebSocketReaderActorState::new(reader, actor_io_sender); //, input_receiver

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
        
                            break;
        
                        }
                        
                    }
                    OpCode::Ping =>
                    {

                        //Now send a pong frame.

                        of.pong_setup();
                        
                        if let Err(_err) = self.io_server.output_sender().send(SimpleWebSocketActorInputMessage::WriteFrame(of)).await
                        {
        
                            break;
        
                        }

                    }
                    OpCode::Continuation | OpCode::Text | OpCode::Binary | OpCode::Pong =>
                    {

                        //If for some reason you get another type of frame, or a Pong...

                        //Send next
                        
                        if let Err(_err) = self.io_server.output_sender().send(SimpleWebSocketActorInputMessage::WriteFrame(of)).await
                        {

                            break;

                        }
    
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

                            if let Err(_err) = self.actor_io_sender.send(of).await
                            {

                                break;
    
                            }

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

                    //You should get this branch after the connection closure procedure has been completed.

                    break;

                }

            }

        }

        false

    }


}

impl_mac_task_actor!(SimpleWebSocketReaderActor);

