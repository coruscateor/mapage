use std::ops::Index;

use corlib::text::SendableText;

use serde_json::Value;

use super::{CommandError, TypeInstance};

pub fn into_bool(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
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

pub fn into_char(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::String(val) = value
    {

        if val.is_empty()
        {

            return Err(CommandError::with_sub_index_opt(SendableText::Str("Char convertion failed: the provided String must have at lest one value."), command_id, field, index, sub_index))

        }

        let mut char_val: char = ' ';

        //Just get the first value.

        for item in val.chars()
        {

            char_val = item;

            break;

        }

        //let char_val = val.chars()[0];

        Ok(TypeInstance::Char(char_val))

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("Char convertion failed"), command_id, field, index, sub_index))
        
    }
    
}