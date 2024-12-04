use std::{error::Error, fmt::Display};

use corlib::text::SendableText;

use serde::{Serialize, Deserialize};

use crate::Command; //types::json::Indices,

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandError
{

    pub id: Option<u32>,
    pub message: SendableText,
    pub index: Option<usize>,
    //pub field: Option<&'static str>,
    //pub indices: Option<Indices>
    pub found_type: Option<SendableText>

}

impl CommandError
{

    pub fn new(command: &Command, message: SendableText) -> Self //id: Option<u32>, //, field: Option<&'static str>, indices: Option<Indices>) -> Self
    {

        Self
        {

            id: command.id,
            message,
            index: None,
            //field,
            //indices
            found_type: None

        }

    }

    pub fn at_index(command: &Command, message: SendableText, index: usize) -> Self //id: Option<u32>, 
    {

        Self
        {

            id: command.id,
            message,
            index:Some(index),
            found_type: None

        }

    }

    pub fn at_index_with_found_type(command: &Command, message: SendableText, index: usize, found_type: SendableText) -> Self
    {

        Self
        {

            id: command.id,
            message,
            index: Some(index),
            found_type: Some(found_type)

        }

    }
    
    /*
    pub fn with_id(id: Option<u32>, message: SendableText) -> Self
    {

        Self
        {

            id,
            message,
            index: None,
            value: None

        }

    }

    pub fn with_id_at_index(id: Option<u32>, message: SendableText, index: usize) -> Self
    {

        Self
        {

            id,
            message,
            index:Some(index),
            value: None

        }

    }
    */

    pub fn invalid_command(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Invalid command provided.")) //command.id,

    }

    /*
    pub fn command_must_have_type(command: &Command) -> Self
    {

        CommandError::new(command.id, SendableText::Str("The provided command must have a type associated with it."))

    }
    */

    pub fn not_implemented(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Not implemented"))

    }

    pub fn invalid_command_for_the_specified_type(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Invalid command for the specified type."))

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


