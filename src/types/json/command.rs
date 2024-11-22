use core::num;
use std::fmt::Display;
use std::{default, string};

//use anyhow::Ok;

//use anyhow::Error;

use std::error::Error;

use serde::{Deserialize, Serialize};

use serde_json::{Map, Number, Value};

use corlib::text::SendableText;

use corlib::inc_dec::{self, IncDecSelf}; //, IntIncDecSelf};

use crate::types::{SupportedType, TypeInstance, Whatever};

use crate::{from_type_instance_type, Command, CommandError};

use super::conversion::*;

use corlib::collections::StackedVec;

impl From<TypeInstance> for bool
{
    
    fn from(value: TypeInstance) -> Self
    {

        if let TypeInstance::Bool(val) = value
        {

            val
            
        }
        else
        {

            panic!("Error: Invalid conversion");
            
        }
       
    }

}

from_type_instance_type!(char, Char);

from_type_instance_type!(f32, F32);

from_type_instance_type!(f64, F64);

from_type_instance_type!(i8, I8);

from_type_instance_type!(i16, I16);

from_type_instance_type!(i32, I32);

from_type_instance_type!(i64, I64);

from_type_instance_type!(i128, I128);

from_type_instance_type!(u8, U8);

from_type_instance_type!(u16, U16);

from_type_instance_type!(u32, U32);

from_type_instance_type!(u64, U64);

from_type_instance_type!(u128, U128);

macro_rules! from_type_instance_type_uc
{

    ($ti_type:ident) =>
    {

        impl From<TypeInstance> for $ti_type
        {
            
            fn from(value: TypeInstance) -> Self
            {

                if let TypeInstance:: $ti_type (val) = value
                {

                    val
                    
                }
                else
                {

                    panic!("Error: Invalid conversion");
                    
                }
            
            }

        }

    }

}

from_type_instance_type_uc!(String);

from_type_instance_type_uc!(Whatever);

/*
impl From<bool> for TypeInstance
{

    fn from(value: bool) -> Self
    {

        TypeInstance::Bool(value)
        
    }

}

impl From<char> for TypeInstance
{

    fn from(value: char) -> Self
    {

        TypeInstance::Char(value)
        
    }

}

impl From<f32> for TypeInstance
{

    fn from(value: f32) -> Self
    {

        TypeInstance::F32(value)
        
    }

}

impl From<f64> for TypeInstance
{

    fn from(value: f64) -> Self
    {

        TypeInstance::F64(value)
        
    }

}

impl From<i8> for TypeInstance
{

    fn from(value: i8) -> Self
    {

        TypeInstance::I8(value)
        
    }

}

impl From<i16> for TypeInstance
{

    fn from(value: i16) -> Self
    {

        TypeInstance::I16(value)
        
    }

}

impl From<i32> for TypeInstance
{

    fn from(value: i32) -> Self
    {

        TypeInstance::I32(value)
        
    }

}

impl From<i64> for TypeInstance
{

    fn from(value: i64) -> Self
    {

        TypeInstance::I64(value)
        
    }

}

impl From<i128> for TypeInstance
{

    fn from(value: i128) -> Self
    {

        TypeInstance::I128(value)
        
    }

}

impl From<u8> for TypeInstance
{

    fn from(value: u8) -> Self
    {

        TypeInstance::U8(value)
        
    }

}

impl From<u16> for TypeInstance
{

    fn from(value: u16) -> Self
    {

        TypeInstance::U16(value)
        
    }

}

impl From<u32> for TypeInstance
{

    fn from(value: u32) -> Self
    {

        TypeInstance::U32(value)
        
    }

}

impl From<u64> for TypeInstance
{

    fn from(value: u64) -> Self
    {

        TypeInstance::U64(value)
        
    }

}

impl From<u128> for TypeInstance
{

    fn from(value: u128) -> Self
    {

        TypeInstance::U128(value)
        
    }

}

impl From<String> for TypeInstance
{

    fn from(value: String) -> Self
    {

        TypeInstance::String(value)
        
    }

}

impl From<Whatever> for TypeInstance
{

    fn from(value: Whatever) -> Self
    {

        TypeInstance::Whatever(value)
        
    }

}
*/

/*
impl TryFrom<Whatever> for TypeInstance
{

    type Error = &'static str;

    fn try_from(value: Whatever) -> Result<Self, Self::Error>
    {

        match value
        {
            Whatever::Bool(val) => Ok(TypeInstance::Bool(val)),
            Whatever::Char(val) => Ok(TypeInstance::Char(val)),
            Whatever::F32(val) => Ok(TypeInstance::F32(val)),
            Whatever::F64(val) => Ok(TypeInstance::F64(val)),
            Whatever::I8(val) => Ok(TypeInstance::I8(val)),
            Whatever::I16(val) => Ok(TypeInstance::I16(val)),
            Whatever::I32(val) => Ok(TypeInstance::I32(val)),
            Whatever::I64(val) => Ok(TypeInstance::I64(val)),
            Whatever::I128(val) => Ok(TypeInstance::I128(val)),
            Whatever::Isize(val) => Ok(TypeInstance::Isize(val)),
            Whatever::U8(val) => Ok(TypeInstance::U8(val)),
            Whatever::U16(val) => Ok(TypeInstance::U16(val)),
            Whatever::U32(val) => Ok(TypeInstance::U32(val)),
            Whatever::U64(val) => Ok(TypeInstance::U64(val)),
            Whatever::U128(val) => Ok(TypeInstance::U128(val)),
            Whatever::USize(val) => Ok(TypeInstance::Usize(val)),
            Whatever::String(val) => Ok(TypeInstance::String(val)),
            Whatever::VecBool(vec) => Ok(TypeInstance::VecBool(vec)),
            Whatever::VecChar(vec) => Ok(TypeInstance::VecChar(vec)),
            Whatever::VecF32(vec) => Ok(TypeInstance::VecF32(vec)),
            Whatever::VecF64(vec) => Ok(TypeInstance::VecF64(vec)),
            Whatever::VecI8(vec) => Ok(TypeInstance::VecI8(vec)),
            Whatever::VecI16(vec) => Ok(TypeInstance::VecI16(vec)),
            Whatever::VecI32(vec) => Ok(TypeInstance::VecI32(vec)),
            Whatever::VecI64(vec) => Ok(TypeInstance::VecI64(vec)),
            Whatever::VecI128(vec) => Ok(TypeInstance::VecI128(vec)),
            Whatever::VecISize(vec) => Ok(TypeInstance::VecISize(vec)),
            Whatever::VecU8(vec) => Ok(TypeInstance::VecU8(vec)),
            Whatever::VecU16(vec) => Ok(TypeInstance::VecU16(vec)),
            Whatever::VecU32(vec) => Ok(TypeInstance::VecU32(vec)),
            Whatever::VecU64(vec) => Ok(TypeInstance::VecU64(vec)),
            Whatever::VecU128(vec) => Ok(TypeInstance::VecU128(vec)),
            Whatever::VecUSize(vec) => Ok(TypeInstance::VecUSize(vec)),
            Whatever::VecString(vec) => Ok(TypeInstance::VecString(vec)),
            _ =>
            {

                Err("Conversion Error")

            }

        }
        
    }

}
*/



/*
//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Default)]
pub struct Command
{

    pub id: Option<u32>,
    pub command: String, //Optional when namespaces get added.
    pub type_name: Option<SupportedType>,
    //namespace: Option<String>,
    pub params: Option<Vec<Option<TypeInstance>>>
    
}
*/

#[derive(Debug)]
pub struct CommandInterpretationError
{

    pub id: Option<u32>,
    pub message: SendableText,
    pub field: Option<&'static str>,
    //indexs: StackedVec<usize, 4>
    pub indices: Option<Indices>
    /*
    index: Option<usize>,
    sub_index: Option<usize>,
    sub_index_2: Option<usize>
    */

}

impl CommandInterpretationError
{

    pub fn new(message: SendableText, id: Option<u32>, field: Option<&'static str>, indices: Option<Indices>) -> Self //Option<&Indices>) -> Self
    {

        /*
        let actual_indices;

        match indices
        {
            Some(val) =>
            {

                actual_indices = Some(val.clone());

            }
            None => actual_indices = None

        }
        */

        Self
        {

            id,
            message,
            field,
            indices //: actual_indices
            //indices: None
            //indexs: StackedVec::new()
            /*
            index: None,
            sub_index: None,
            sub_index_2: None
            */
        }

    }

    /*
    pub fn with_index(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: usize) -> Self
    {

        let mut indices = StackedVec::new();

        if indices.push(index).is_some()
        {

            panic!("This should've worked!");

        }

        Self
        {

            id,
            message,
            field, //: field,
            indices: Some(indices)
            /*
            index: Some(index),
            sub_index: None,
            sub_index_2: None
            */
        }

    }

    pub fn with_indexs(message: SendableText, id: Option<u32>, field: Option<&'static str>, indices: &StackedVec<usize, 4>) -> Self
    {

        Self
        {

            id,
            message,
            field,
            indices: Some(indices.clone())

        }

    }
    */

    /*
    pub fn with_sub_index(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: usize, sub_index: usize) -> Self
    {

        Self
        {

            id,
            message,
            field: field,
            index: Some(index),
            sub_index: Some(sub_index),
            sub_index_2: None

        }

    }

    pub fn with_sub_index_2(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: usize, sub_index: usize, sub_index_2: usize) -> Self
    {

        Self
        {

            id,
            message,
            field: field,
            index: Some(index),
            sub_index: Some(sub_index),
            sub_index_2: Some(sub_index_2)

        }

    }

    pub fn with_index_opt(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: Option<usize>) -> Self
    {

        Self
        {

            id,
            message,
            field,
            index,
            sub_index: None,
            sub_index_2: None

        }

    }

    pub fn with_sub_index_opt(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>) -> Self
    {

        Self
        {

            id,
            message,
            field,
            index,
            sub_index,
            sub_index_2: None

        }

    }

    pub fn with_sub_index_2_opt(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: Option<usize>, sub_index: Option<usize>, sub_index_2: Option<usize>) -> Self
    {

        Self
        {

            id,
            message,
            field,
            index,
            sub_index,
            sub_index_2

        }

    }
    */

}

impl Display for CommandInterpretationError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "Message: {}, Id: {:#?}", self.message, self.id)       

    }

}

impl Error for CommandInterpretationError
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

//JSON

/*
fn convert_number_from_vec(number: Number, index: usize, command: &Command, field: Option<&'static str>) -> Result<TypeInstance, CommandError>
{

    if number.is_f64()
    {

        match number.as_f64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::F64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to f64.", index)), command.id, field))

            }

        }

    }
    else if number.is_i64()
    {

        match number.as_i64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::I64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to i64.", index)), command.id, field))

            }

        }

    }
    else if number.is_u64()
    {

        match number.as_u64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::U64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to u64.", index)), command.id, field))

            }

        }
        
    }
    else
    {

        Err(CommandError::new(SendableText::String(format!("Internal error converting param at index: {} to unknown.", index)), command.id, field))
        
    }

}
*/

fn convert_number(number: Number, command: &Command, field: Option<&'static str>, indices: Option<Indices>) -> Result<TypeInstance, CommandInterpretationError>
{

    if number.is_f64()
    {

        match number.as_f64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::F64(val))

            }
            None =>
            {

                Err(CommandInterpretationError::new(SendableText::Str("Unexpected error converting param to f64."), command.id, field, indices))

            }

        }

    }
    else if number.is_i64()
    {

        match number.as_i64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::I64(val))

            }
            None =>
            {

                Err(CommandInterpretationError::new(SendableText::Str("Unexpected error converting param to i64."), command.id, field, indices))

            }

        }

    }
    else if number.is_u64()
    {

        match number.as_u64()
        {

            Some(val) =>
            {

                Ok(TypeInstance::U64(val))

            }
            None =>
            {

                Err(CommandInterpretationError::new(SendableText::Str("Unexpected error converting param to u64."), command.id, field, indices))

            }

        }
        
    }
    else
    {

        Err(CommandInterpretationError::new(SendableText::Str("Internal error converting param to unknown."), command.id, field, indices))
        
    }

}

//Sub Vec/Array

/*
fn convert_number_from_sub_vec(number: Number, index: usize, command: &Command, field: Option<&'static str>) -> Result<Whatever, CommandError>
{


    if number.is_f64()
    {

        match number.as_f64()
        {

            Some(val) =>
            {

                Ok(Whatever::F64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to f64.", index)), command.id, field))

            }

        }

    }
    else if number.is_i64()
    {

        match number.as_i64()
        {

            Some(val) =>
            {

                Ok(Whatever::I64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to i64.", index)), command.id, field))

            }

        }

    }
    else if number.is_u64()
    {

        match number.as_u64()
        {

            Some(val) =>
            {

                Ok(Whatever::U64(val))

            }
            None =>
            {

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to u64.", index)), command.id, field))

            }

        }
        
    }
    else
    {

        Err(CommandError::new(SendableText::String(format!("Internal error converting param at index: {} to unknown.", index)), command.id, field))
        
    }

}
*/

//Process a Map as a set of parameters for a command.

fn process_map(map: Map<String, Value>, command: &Command, field: Option<&'static str>, indices: &Option<Indices>, parsed_params: &mut Vec<Option<TypeInstance>>) -> Result<(), CommandInterpretationError> //Result<Vec<TypeInstance>, CommandError> //index_opt: Option<usize>, 
{

    if !map.is_empty()
    {

        //let mut res_vec = Vec::with_capacity(map.len());

        let mut index: usize = 0;

        for (key, value) in map
        {

            match key.as_str() //.trim()
            {
    
                "none" =>
                {

                    parsed_params.push(None);

                }
                "type_bool" => 
                {

                    parsed_params.push(Some(into_bool(value, command.id, field, indices)?)); //index_opt, Some(index))?);

                }
                "type_char" => 
                {

                    parsed_params.push(Some(into_char(value, command.id, field, indices)?)); //index_opt, Some(index))?);

                }
                "type_f32" => 
                {

                    parsed_params.push(Some(into_f32(value, command.id, field, indices)?));

                }
                "type_f64" => 
                {

                    parsed_params.push(Some(into_f64(value, command.id, field, indices)?));

                }
                "type_i8" => 
                {

                    parsed_params.push(Some(into_i8(value, command.id, field, indices)?));

                }
                "type_i16" => 
                {

                    parsed_params.push(Some(into_i16(value, command.id, field, indices)?));

                }
                "type_i32" => 
                {

                    parsed_params.push(Some(into_i32(value, command.id, field, indices)?));

                }
                "type_i64" => 
                {

                    parsed_params.push(Some(into_i64(value, command.id, field, indices)?));

                }
                "type_i128" => 
                {

                    parsed_params.push(Some(into_i128(value, command.id, field, indices)?));

                }
                /*
                "type_isize" => 
                {

                    

                }
                */
                "type_u8" => 
                {

                    parsed_params.push(Some(into_u8(value, command.id, field, indices)?));

                }
                "type_u16" => 
                {

                    parsed_params.push(Some(into_u16(value, command.id, field, indices)?));

                }
                "type_u32" => 
                {

                    parsed_params.push(Some(into_u32(value, command.id, field, indices)?));

                }
                "type_u64" => 
                {

                    parsed_params.push(Some(into_u64(value, command.id, field, indices)?));

                }
                "type_u128" => 
                {

                    parsed_params.push(Some(into_u128(value, command.id, field, indices)?));

                }
                /*
                "type_usize" => 
                {

                    

                }
                */
                "type_string" => 
                {

                    parsed_params.push(Some(into_string(value, command.id, field, indices)?));

                }
                "type_whatever" => 
                {

                    parsed_params.push(Some(into_whatever(value, command.id, field, indices)?));

                }
                "type_vec_bool" => 
                {

                    parsed_params.push(Some(into_vec_bool(value, command.id, field, indices)?));

                }
                /*
                "type_vec_char" => 
                {

                    parsed_params.push(into_vec_char(value, command.id, field, indices)?);

                }
                */
                "type_vec_f32" => 
                {

                    parsed_params.push(Some(into_vec_f32(value, command.id, field, indices)?));

                }
                "type_vec_f64" => 
                {

                    parsed_params.push(Some(into_vec_f64(value, command.id, field, indices)?));

                }
                "type_vec_i8" => 
                {

                    parsed_params.push(Some(into_vec_i8(value, command.id, field, indices)?));

                }
                "type_vec_i16" => 
                {

                    parsed_params.push(Some(into_vec_i16(value, command.id, field, indices)?));

                }
                "type_vec_i32" => 
                {

                    parsed_params.push(Some(into_vec_i32(value, command.id, field, indices)?));

                }
                "type_vec_i64" => 
                {

                    parsed_params.push(Some(into_vec_i64(value, command.id, field, indices)?));

                }
                "type_vec_i128" => 
                {

                    parsed_params.push(Some(into_vec_i128(value, command.id, field, indices)?));

                }
                /*
                "type_vec_isize" => 
                {

                    

                }
                */
                "type_vec_u8" => 
                {

                    parsed_params.push(Some(into_vec_u8(value, command.id, field, indices)?));

                }
                "type_vec_u16" => 
                {

                    parsed_params.push(Some(into_vec_u16(value, command.id, field, indices)?));

                }
                "type_vec_u32" => 
                {

                    parsed_params.push(Some(into_vec_u32(value, command.id, field, indices)?));

                }
                "type_vec_u64" => 
                {

                    parsed_params.push(Some(into_vec_u64(value, command.id, field, indices)?));

                }
                "type_vec_u128" => 
                {

                    parsed_params.push(Some(into_vec_u128(value, command.id, field, indices)?));

                }
                /*
                "type_vec_usize" => 
                {

                    

                }
                */
                "type_vec_string" => 
                {

                    parsed_params.push(Some(into_vec_string(value, command.id, field, indices)?));

                }
                "type_vec_whatever" => 
                {

                    parsed_params.push(Some(into_vec_whatever(value, command.id, field, indices)?));

                }
                _ =>
                {

                    return Err(CommandInterpretationError::new(SendableText::Str("Internal error connot convert a parameter at index to unknown type."), command.id, field, indices.clone()));

                    //return Err(CommandError::with_index_opt(SendableText::Str("Internal error converting param at index to unknown."), command.id, field, index_opt));
    
                    /*
                    match index_opt
                    {
    
                        Some(index) =>
                        {
    
                            return Err(CommandError::new(SendableText::String(format!("Internal error converting param at index: {} to unknown.", index)), command.id, field));
    
                        }
                        None =>
                        {
    
                            return Err(CommandError::new(SendableText::Str("Internal error converting param at index to unknown."), command.id, field));
    
                        }
                        
                    }
                    */
    
                }
                
            }

            index.pp();
    
        } 

        Ok(())

        //Ok(parsed_params)

    }
    else
    {

        Err(CommandInterpretationError::new(SendableText::Str("Empty Map not expected."), command.id, field, indices.clone()))
        
    }

}

//pub static sv_size: usize = 4; 

pub const SV_SIZE: usize = 4;

pub type Indices = StackedVec::<usize, SV_SIZE>;

//Start here

pub fn into_command(input: Value) -> Result<Command, CommandInterpretationError>
{

    let mut indices = Indices::new(); //StackedVec::<usize, SV_SIZE>::new(); //For error reporting

    let error_message;

    match input
    {

        Value::Null =>
        {

            error_message = "A null value is an invalid command";

        }
        Value::Bool(_) =>
        {

            error_message = "A bool value is an invalid command";

        }
        Value::Number(_number) =>
        {

            error_message = "A number is an invalid command";

        }
        Value::String(_) =>
        {

            error_message = "A String is an invalid command";

        }
        Value::Array(_vec) =>
        {

            error_message = "Processing sets of commands is not currently supported";

        }
        Value::Object(mut map) =>
        {

            let mut command = Command::default();

            //let id_opt = map.get("id");

            let field = "id";

            let id_opt = map.remove(field);

            if let Some(val) = id_opt
            {

                //let id_res: Result<u32, _> = id_val.try_into(); //u32::try_from(id_val); //.into();

                let as_number = val.as_number();

                if let Some(number) = as_number
                {

                    match number.as_u64() //.into();
                    {

                        Some(val) =>
                        {

                            command.id = Some(val as u32);

                        }
                        None =>
                        {

                            return Err(CommandInterpretationError::new(SendableText::Str("Integer expected in id field."), None, Some(field), None));

                        }

                    }

                }
                else
                {

                    return Err(CommandInterpretationError::new(SendableText::Str("Integer expected in id field."), None, Some(field), None));                    
                    
                }

            }

            //let command_opt = map.get("command");

            let field = "command";

            let command_opt = map.remove(field);

            if let Some(val) = command_opt
            {

                match val
                {

                    Value::String(string_val) =>
                    {

                        command.command = string_val;

                    }
                    _ =>
                    {

                        return Err(CommandInterpretationError::new(SendableText::Str("The command field must be a String."), command.id, Some(field), None));  

                    }
                    
                }

                /*
                if val.is_string()
                {

                    let value: String = val.into();

                }
                else
                {
      

                }
                */

            }
            else
            {

                return Err(CommandInterpretationError::new(SendableText::Str("Command field not found."), command.id, Some(field), None));     
                
            }

            let field = "type";

            let type_opt = map.remove(field);

            {

                //https://doc.rust-lang.org/error_codes/E0597.html

                //let type_opt = map.get("type");

                if let Some(val) = type_opt
                {

                    match val
                    {

                        Value::String(string_val) =>
                        {

                            //let trimmed = string_val.trim();

                            match SupportedType::try_parse(&string_val) //trimmed)
                            {

                                Ok(st) =>
                                {

                                    command.type_name = Some(st);

                                }
                                Err(err) =>
                                {

                                    return Err(CommandInterpretationError::new(err, command.id, Some(field), None));

                                    //return Err(SendableText::Str(err)); 

                                }

                            }

                        }
                        Value::Null =>
                        {

                            command.type_name = None;

                        }
                        _ =>
                        {

                            return Err(CommandInterpretationError::new(SendableText::Str("The command field must be a String."), command.id, Some(field), None));  

                        }
                        
                    }

                }
                else
                {

                    command.type_name = None;

                    //return Err(CommandError::new(SendableText::Str("A type must be specified."), command.id, Some(field), None));  
                    
                }

            }

            //Params array...

            let field = "params";

            let params_opt = map.remove(field);

            match params_opt
            {

                Some(params) =>
                {

                    match params
                    {

                        Value::Null =>
                        {

                            //Single parameter

                            command.params = None;

                        }
                        Value::Bool(val) =>
                        {

                            //Single parameter

                            let mut params_vec = Vec::with_capacity(1);

                            params_vec.push(Some(TypeInstance::Bool(val)));

                            command.params = Some(params_vec);

                            //return Err(CommandError::new(SendableText::Str("The params field cannot be a bool value."), command.id));  

                        }
                        Value::Number(val) =>
                        {

                            //Single parameter

                            let mut params_vec = Vec::with_capacity(1);

                            let number = convert_number(val, &command, Some(field), None)?;

                            params_vec.push(Some(number));

                            command.params = Some(params_vec);

                            //return Err(CommandError::new(SendableText::Str("The params field cannot be a number value."), command.id));  

                        }
                        Value::String(val) =>
                        {

                            //Single parameter

                            let mut params_vec = Vec::with_capacity(1);

                            params_vec.push(Some(TypeInstance::String(val)));

                            command.params = Some(params_vec);

                            //return Err(CommandError::new(SendableText::Str("The params field cannot be a String."), command.id, Some(field)));  

                        }
                        Value::Array(vec) =>
                        {

                            //Multiple parameters (maybe)

                            if vec.is_empty()
                            {

                                command.params = None;

                            }
                            else
                            {
                                
                                let mut params_vec = Vec::with_capacity(vec.len());

                                //let mut index: usize = 0;

                                indices.push(0);

                                for mut item in vec
                                {

                                    let taken_item = item.take();

                                    match taken_item
                                    {

                                        Value::Null =>
                                        {

                                            params_vec.push(None);

                                        }
                                        Value::Bool(val) =>
                                        {

                                            params_vec.push(Some(TypeInstance::Bool(val)));

                                        }
                                        Value::Number(number) =>
                                        {

                                            //indices.push(number);

                                            match convert_number(number, &command, Some(field), Some(indices)) //convert_number_from_vec(number, index, &command, Some(field))
                                            {

                                                Ok(res) =>
                                                {

                                                    params_vec.push(Some(res));

                                                }
                                                Err(err) =>
                                                {

                                                    return Err(err);

                                                }

                                            }

                                            /*
                                            if number.is_f64()
                                            {

                                                match number.as_f64()
                                                {

                                                    Some(val) =>
                                                    {

                                                        params_vec.push(Some(TypeInstance::F64(val)));

                                                    }
                                                    None =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to f64.", index)), command.id));

                                                    }

                                                }

                                            }
                                            else if number.is_i64()
                                            {

                                                match number.as_i64()
                                                {

                                                    Some(val) =>
                                                    {

                                                        params_vec.push(Some(TypeInstance::I64(val)));

                                                    }
                                                    None =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to i64.", index)), command.id));

                                                    }

                                                }

                                            }
                                            else if number.is_u64()
                                            {

                                                match number.as_u64()
                                                {

                                                    Some(val) =>
                                                    {

                                                        params_vec.push(Some(TypeInstance::U64(val)));

                                                    }
                                                    None =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to u64.", index)), command.id));

                                                    }

                                                }
                                                
                                            }
                                            */

                                        }
                                        Value::String(val) =>
                                        {

                                            params_vec.push(Some(TypeInstance::String(val)));

                                        }
                                        Value::Array(_vec_val) =>
                                        {

                                            return Err(CommandInterpretationError::new(SendableText::Str("Processing arrays without accompanying type information is not supported."), command.id, Some(field), Some(indices)));

                                            //Sub array

                                            /*
                                            let mut vec_param = Vec::with_capacity(vec_val.len());

                                            let mut sub_index: usize = 0;

                                            for mut item in vec_val
                                            {

                                                let taken_item = item.take();

                                                match taken_item
                                                {

                                                    Value::Null =>
                                                    {

                                                        vec_param.push(None);

                                                    }
                                                    Value::Bool(val) =>
                                                    {

                                                        vec_param.push(Some(Whatever::Bool(val)));

                                                    }
                                                    Value::Number(number) =>
                                                    {

                                                        match convert_number_from_sub_vec(number, index, &command, Some(field))
                                                        {
            
                                                            Ok(res) =>
                                                            {
            
                                                                vec_param.push(Some(res));
            
                                                            }
                                                            Err(err) =>
                                                            {
            
                                                                return Err(err);
            
                                                            }
            
                                                        }

                                                    }
                                                    Value::String(val) =>
                                                    {

                                                        vec_param.push(Some(Whatever::String(val)));

                                                    }
                                                    Value::Array(_vec) =>
                                                    {

                                                        //return Err(CommandError::new(SendableText::String(format!("Array at param index: {}, sub-index: {}. Arrays of arrays are not supported.", index, sub_index)), command.id, Some(field)));

                                                    }
                                                    Value::Object(_map) =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Map at param index: {}, sub-index: {}. Map params are not supported.", index, sub_index)), command.id, Some(field)));

                                                    }

                                                }

                                                sub_index.pp();

                                                //sub_index.overflowing_pp();

                                            }

                                            params_vec.push(Some(TypeInstance::VecOptionWhatever(vec_param)))
                                            */

                                        }
                                        Value::Object(map) =>
                                        {

                                            //Multiple type annotated Values in addition to any other Value objects.

                                            //Should probably adjust the capacity.

                                            process_map(map, &command, Some(field), &Some(indices), &mut params_vec)?; //index_opt, 

                                            //return Err(CommandError::new(SendableText::String(format!("Map at param index: {}. Map params are not supported.", index)), command.id));

                                        }

                                    }

                                    let index= indices.last_mut().expect("Error: There should be a value here.");
                                    
                                    index.pp();
                                    
                                }

                                indices.pop();

                                command.params = Some(params_vec);

                            }

                        }
                        Value::Object(map) =>
                        {

                            let mut params_vec = Vec::with_capacity(map.len());

                            //Multiple type annotated Values only.

                            process_map(map, &command, Some(field), &Some(indices), &mut params_vec)?;

                            command.params = Some(params_vec);

                            //return Err(CommandError::new(SendableText::Str("The params field cannot be an Object."), command.id)); 

                        }

                    }

                    //let parsed_params: Vec<Option<TypeInstance>> = Vec::new();



                }
                None =>
                {

                    command.params = None;

                    //if let?

                }

            }

            return Ok(command);

            //error_message = "I don't know WTF happened.";

        }

    }

    //Ok(command)

    Err(CommandInterpretationError::new(SendableText::Str(error_message), None, None, Some(indices)))

}

