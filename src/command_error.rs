use corlib::text::SendableText;

use serde::Serialize;

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

}
