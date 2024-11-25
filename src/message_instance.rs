use corlib::text::SendableText;

use crate::{Command, CommandError, CommandResult};


pub enum MessageInstance
{

    Command(Command),
    CommandResult(CommandResult),
    CommandError(CommandError),
    Error(SendableText)

}


