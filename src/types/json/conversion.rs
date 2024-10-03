use std::ops::Index;

//use anyhow::Ok;
use corlib::text::SendableText;

use dashmap::mapref::entry;
use serde_json::Value;

use crate::types::Whatever;

use super::{CommandError, TypeInstance};

use paste::paste;

use std::str::FromStr;

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

            return Err(CommandError::with_sub_index_opt(SendableText::Str("Char conversion failed: the provided String must have at least one value."), command_id, field, index, sub_index))

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

        Err(CommandError::with_sub_index_opt(SendableText::Str("Char conversion failed"), command_id, field, index, sub_index))
        
    }
    
}

pub fn into_f32(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Number(val) = value
    {

        if let Some(f64_val) = val.as_f64()
        {

            Ok(TypeInstance::F32(f64_val as f32))

        }
        else if let Some(i64_val) = val.as_i64()
        {


            Ok(TypeInstance::F32(i64_val as f32))

        }
        else if let Some(u64_val) = val.as_u64()
        {


            Ok(TypeInstance::F32(u64_val as f32))

        }
        else
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))    
            
        }

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))
        
    }

}

pub fn into_f64(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Number(val) = value
    {

        if let Some(f64_val) = val.as_f64()
        {

            Ok(TypeInstance::F64(f64_val))

        }
        else if let Some(i64_val) = val.as_i64()
        {

            Ok(TypeInstance::F64(i64_val as f64))

        }
        else if let Some(u64_val) = val.as_u64()
        {

            Ok(TypeInstance::F64(u64_val as f64))

        }
        else
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))    
            
        }

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))
        
    }

}

pub fn into_i8(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Number(val) = value
    {

        if let Some(f64_val) = val.as_f64()
        {

            Ok(TypeInstance::I8(f64_val as i8))

        }
        else if let Some(i64_val) = val.as_i64()
        {
            
            Ok(TypeInstance::I8(i64_val as i8))

        }
        else if let Some(u64_val) = val.as_u64()
        {

            Ok(TypeInstance::I8(u64_val as i8))

        }
        else
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))    
            
        }

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("F32 conversion failed"), command_id, field, index, sub_index))
        
    }

}

#[macro_export]
macro_rules! into_type_instance_number
{

    ($number_type:ident, $ti_number_type:ident) =>
    {

        paste!
        {

            pub fn [<into_ $number_type>](value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
            {

                use TypeInstance::*;

                if let Value::Number(val) = value
                {

                    if let Some(f64_val) = val.as_f64()
                    {

                        Ok($ti_number_type(f64_val as $number_type))

                    }
                    else if let Some(i64_val) = val.as_i64()
                    {

                        Ok($ti_number_type (i64_val as $number_type))
                        
                        //[<Ok(TypeInstance:: $ti_number_type (i64_val as $number_type))>]

                    }
                    else if let Some(u64_val) = val.as_u64()
                    {

                        Ok($ti_number_type(u64_val as $number_type))

                        //[<Ok(TypeInstance:: $ti_number_type (u64_val as $number_type))>]

                    }
                    else
                    {

                        Err(CommandError::with_sub_index_opt(SendableText::Str("$ti_number_type conversion failed"), command_id, field, index, sub_index))    
                        
                    }

                }
                else
                {

                    Err(CommandError::with_sub_index_opt(SendableText::Str("$ti_number_type conversion failed"), command_id, field, index, sub_index))
                    
                }

            }

        }

    } 

}

into_type_instance_number!(i16, I16);

into_type_instance_number!(i32, I32);

into_type_instance_number!(i64, I64);

//Can be a String.

pub fn into_i128(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Number(val) = value
    {

        if let Some(f64_val) = val.as_f64()
        {

            Ok(TypeInstance::I128(f64_val as i128))

        }
        else if let Some(i64_val) = val.as_i64()
        {
            
            Ok(TypeInstance::I128(i64_val as i128))

        }
        else if let Some(u64_val) = val.as_u64()
        {

            Ok(TypeInstance::I128(u64_val as i128))

        }
        else
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("I128 conversion failed"), command_id, field, index, sub_index))    
            
        }

    }
    else if let Value::String(val) = value
    {

        match i128::from_str(&val)
        {

            Ok(val) =>
            {

                Ok(TypeInstance::I128(val))

            }
            Err(err) =>
            {

                Err(CommandError::with_sub_index_opt(SendableText::String(err.to_string()), command_id, field, index, sub_index))    

            }

        }

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("I128 conversion failed"), command_id, field, index, sub_index))
        
    }

}

//into_type_instance_number!(isize, Isize);

into_type_instance_number!(u8, U8);

into_type_instance_number!(u16, U16);

into_type_instance_number!(u32, U32);

into_type_instance_number!(u64, U64);

//Can be a String.

pub fn into_u128(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Number(val) = value
    {

        if let Some(f64_val) = val.as_f64()
        {

            Ok(TypeInstance::U128(f64_val as u128))

        }
        else if let Some(i64_val) = val.as_i64()
        {
            
            Ok(TypeInstance::U128(i64_val as u128))

        }
        else if let Some(u64_val) = val.as_u64()
        {

            Ok(TypeInstance::U128(u64_val as u128))

        }
        else
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("I128 conversion failed"), command_id, field, index, sub_index))    
            
        }

    }
    else if let Value::String(val) = value
    {

        match u128::from_str(&val)
        {

            Ok(val) =>
            {

                Ok(TypeInstance::U128(val))

            }
            Err(err) =>
            {

                Err(CommandError::with_sub_index_opt(SendableText::String(err.to_string()), command_id, field, index, sub_index))    

            }

        }

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("U128 conversion failed"), command_id, field, index, sub_index))
        
    }

}

//into_type_instance_number!(usize, Usize);

//Collections etc...

pub fn into_string(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::String(val) = value
    {

        Ok(TypeInstance::String(val))

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("String conversion failed"), command_id, field, index, sub_index))
        
    }
    
}

pub fn into_whatever(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    match value
    {

        Value::Null =>
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("Whatevers cannot be converted from Null values."), command_id, field, index, sub_index))

        }
        Value::Bool(val) =>
        {

            Ok(TypeInstance::Whatever(Whatever::Bool(val)))

        }
        Value::Number(val) =>
        {

            if let Some(f64_val) = val.as_f64()
            {
    
                Ok(TypeInstance::Whatever(Whatever::F64(f64_val)))
    
            }
            else if let Some(i64_val) = val.as_i64()
            {
                
                Ok(TypeInstance::Whatever(Whatever::I64(i64_val)))
    
            }
            else if let Some(u64_val) = val.as_u64()
            {
    
                Ok(TypeInstance::Whatever(Whatever::U64(u64_val)))
                
            }
            else
            {
    
                Err(CommandError::with_sub_index_opt(SendableText::Str("Whatever number conversion failed."), command_id, field, index, sub_index))    
                
            }

        }
        Value::String(val) =>
        {

            Ok(TypeInstance::Whatever(Whatever::String(val)))

        }
        Value::Array(_vec) =>
        {

            Err(CommandError::with_sub_index_opt(SendableText::Str("Conversion failed: Whatever objects cannot contain collections of Whatever objects."), command_id, field, index, sub_index))    

        }
        Value::Object(map) =>
        {

            if !map.is_empty()
            {

                let mut entry = ("".to_string(), Value::Null);

                for item in map
                {

                    entry = item;

                    break;

                }

                let whatever;

                match entry.0.as_str()
                {

                    "type_bool" => 
                    {

                        let res = into_bool(entry.1, command_id, field, index, sub_index)?;

                        //let whatever: Whatever = res.try_into();

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                        //Ok(TypeInstance::Whatever(whatever))

                        /*
                        match res.into_whatever(command_id, field, index, sub_index)
                        {

                            Ok(res) =>
                            {



                            }
                            Err(err) => todo!(),
                            
                        }
                         */

                        //Ok(TypeInstance::Whatever(Whatever::Bool(entry.1)))

                    }
                    "type_char" =>
                    {

                        let res = into_char(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                        //Ok(TypeInstance::Whatever(whatever))

                    }
                    "type_f32" =>
                    {

                        let res = into_f32(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                        //Ok(TypeInstance::Whatever(whatever))

                    }
                    "type_f64" =>
                    {

                        let res = into_f64(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_i8" =>
                    {

                        let res = into_i8(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_i16" =>
                    {

                        let res = into_i16(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_i32" =>
                    {

                        let res = into_i32(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_i64" =>
                    {

                        let res = into_i64(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_i128" =>
                    {

                        let res = into_i128(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    /*
                    "type_isize" =>
                    {

                        let res = into_isize(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    */
                    "type_u8" =>
                    {

                        let res = into_u8(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_u16" =>
                    {

                        let res = into_u16(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_u32" =>
                    {

                        let res = into_u32(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_u64" =>
                    {

                        let res = into_u64(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_u128" =>
                    {

                        let res = into_u128(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_string" =>
                    {

                        let res = into_string(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_whatever" =>
                    {

                        let res = into_whatever(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;

                    }
                    "type_vec_bool" => 
                    {
    
                        let res = into_vec_bool(entry.1, command_id, field, index, sub_index)?;

                        whatever = res.into_whatever(command_id, field, index, sub_index)?;
    
                    }
                    _ =>
                    {

                        return Err(CommandError::with_sub_index_opt(SendableText::Str("Invalid type metadata provided."), command_id, field, index, sub_index));

                    }
                    
                }

                Ok(TypeInstance::Whatever(whatever))

               //let item = map[0];

               //Ok(TypeInstance::Whatever(Whatever::U64(0)))

            }
            else
            {

                Err(CommandError::with_sub_index_opt(SendableText::Str("A Map with at least one entry expected."), command_id, field, index, sub_index))
                
            }

        }

    }

}

pub fn into_vec_bool(value: Value, command_id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Result<TypeInstance, CommandError>
{

    if let Value::Array(arr)= value
    {

        let new_vec = Vec::with_capacity(arr.len());

        for item in arr
        {

            let res = into_bool(item, command_id, field, index, sub_index);

            new_vec.push(value);

        }

        Ok(TypeInstance::VecBool(val))

    }
    else
    {

        Err(CommandError::with_sub_index_opt(SendableText::Str("Bool convertion failed"), command_id, field, index, sub_index))
        
    }
    
}
