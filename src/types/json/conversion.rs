use corlib::text::SendableText;

use serde_json::Value;

use super::{CommandError, TypeInstance};

pub fn into_bool(mut value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Bool(val) = value
    {

        Ok(TypeInstance::Bool(val))

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("Bool convertion failed"), command_id, field, index, sub_index))
        
    }
    
}