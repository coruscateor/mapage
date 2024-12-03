use corlib::text::SendableText;

use crate::{types::TypeInstance, Command};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult
{

    pub id: Option<u32>,
    pub result: Option<TypeInstance>,
    //pub message: Option<SendableText>,
    pub done: bool

}

impl CommandResult
{

    pub fn new(id: Option<u32>, result: Option<TypeInstance>, done: bool) -> Self
    {

        Self
        {

            id,
            result,
            done

        }

    }

    pub fn done(command: &Command, result: Option<TypeInstance>) -> Self
    {

        Self
        {

            id: command.id,
            result,
            done: true

        }

    }
    
    pub fn not_done(command: &Command, result: Option<TypeInstance>) -> Self
    {

        Self
        {

            id: command.id,
            result,
            done: false

        }

    }

}



