use corlib::text::SendableText;

use crate::{types::json::Command, CommandError, CommandResult};


pub enum CommunicationInstance
{

    Command(Command),
    CommandResult(CommandResult),
    CommandError(CommandError),
    Error(SendableText)

}


