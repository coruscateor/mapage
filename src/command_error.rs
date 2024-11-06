use corlib::text::SendableText;

use serde::Serialize;

#[derive(Serialize)]
pub struct CommandError
{

    pub id: Option<u32>,
    pub message: SendableText

}
