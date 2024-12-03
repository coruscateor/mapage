use corlib::text::SendableText;

use serde::{Serialize, Deserialize};

use crate::{Command, CommandError, CommandResult};


#[derive(Debug, Serialize, Deserialize)]
pub enum StreamedMessage
{

    Command(Command),
    CommandResult(CommandResult),
    CommandError(CommandError),
    Error(SendableText)

}


