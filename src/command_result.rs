use corlib::text::SendableText;

use crate::types::json::TypeInstance;


pub struct CommandResult
{

    pub id: Option<u32>,
    pub result: TypeInstance,
    pub message: Option<SendableText>,
    pub is_error: bool,
    pub fin: bool

}

