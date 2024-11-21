use corlib::text::SendableText;

use serde::Serialize;

use crate::types::json::Command;

#[derive(Serialize)]
pub struct CommandError
{

    pub id: Option<u32>,
    pub message: SendableText

}

impl CommandError
{

    pub fn new(id: Option<u32>, message: SendableText) -> Self
    {

        Self
        {

            id,
            message

        }

    }

    pub fn invalid_command(command: &Command) -> Self
    {

        CommandError::new(command.id, SendableText::Str("Invalid command provided."))

    }

    /*
    pub fn command_must_have_type(command: &Command) -> Self
    {

        CommandError::new(command.id, SendableText::Str("The provided command must have a type associated with it."))

    }
    */

    pub fn not_implemented(command: &Command) -> Self
    {

        CommandError::new(command.id, SendableText::Str("Not implemented"))

    }

    pub fn invalid_command_for_the_specified_type(command: &Command) -> Self
    {

        CommandError::new(command.id, SendableText::Str("Invalid command for the specified type."))

    }

}
