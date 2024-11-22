use std::{error::Error, fmt::Display};

use corlib::text::SendableText;

use serde::Serialize;

use crate::{types::json::Indices, Command};

#[derive(Serialize, Debug)]
pub struct CommandError
{

    pub id: Option<u32>,
    pub message: SendableText,
    //pub field: Option<&'static str>,
    //pub indices: Option<Indices>

}

impl CommandError
{

    pub fn new(id: Option<u32>, message: SendableText) -> Self //, field: Option<&'static str>, indices: Option<Indices>) -> Self
    {

        Self
        {

            id,
            message,
            //field,
            //indices

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

impl Display for CommandError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "Message: {}, Id: {:#?}", self.message, self.id)       

    }

}

impl Error for CommandError
{

    fn source(&self) -> Option<&(dyn Error + 'static)>
    {

        None

    }

    fn description(&self) -> &str
    {

        "description() is deprecated; use Display"

    }

    fn cause(&self) -> Option<&dyn Error>
    {

        self.source()

    }

    //fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}


