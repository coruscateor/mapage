use std::fmt::Display;
use std::{default, string};

//use anyhow::Ok;

//use anyhow::Error;

use std::error::Error;

use serde::{Deserialize, Serialize};

use serde_json::{Map, Number, Value};

use corlib::text::SendableText;

use corlib::inc_dec::{self, IncDecSelf}; //, IntIncDecSelf};

use crate::types::Whatever;

use super::conversion::{into_bool, into_char};

#[derive(Debug, Default)]
pub enum SupportedType
{

    #[default]
    Bool,
    Char,

    F32,
    F64,
    I8,
    I16,
    I32,
    I64,

    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,

    U128,
    Usize,

    //Collections

    String,
    Whatever,

    //Vecs

    VecBool,
    VecChar,

    VecF32,
    VecF64,
    VecI8,
    VecI16,
    VecI32,
    VecI64,

    VecI128,
    VecIsize,
    VecU8,
    VecU16,
    VecU32,
    VecU64,

    VecU128,
    VecUsize,

    VecString,
    VecWhatever

}

impl SupportedType
{

    pub fn try_parse(slice: &str) -> Result<SupportedType, SendableText> //&str>
    {

        match slice
        {

            "f32" => Ok(SupportedType::F32),
            "f64" => Ok(SupportedType::F64),
            "i8" => Ok(SupportedType::I8),
            "i16" => Ok(SupportedType::I16),
            "i32" => Ok(SupportedType::I32),
            "i64" => Ok(SupportedType::I64),
            "i128" => Ok(SupportedType::I128),
            "isize" => Ok(SupportedType::Isize),
            "u8" => Ok(SupportedType::U8),
            "u16" => Ok(SupportedType::U16),
            "u32" => Ok(SupportedType::U32),
            "u64" => Ok(SupportedType::U64),
            "u128" => Ok(SupportedType::U128),
            "usize" => Ok(SupportedType::Usize),

    //Collections

            "string" => Ok(SupportedType::String),
            "whatever" => Ok(SupportedType::Whatever),

    //Vecs

            "vec_bool" => Ok(SupportedType::VecBool),
            "vec_char" => Ok(SupportedType::VecChar),
            "vec_f32" => Ok(SupportedType::VecF32),
            "vec_f64" => Ok(SupportedType::VecF64),
            "vec_i8" => Ok(SupportedType::VecI8),
            "vec_i16" => Ok(SupportedType::I16),
            "vec_i32" => Ok(SupportedType::VecI32),
            "vec_i64" => Ok(SupportedType::VecI64),
            "vec_i128" => Ok(SupportedType::VecI128),
            "vec_isize" => Ok(SupportedType::VecIsize),
            "vec_u8" => Ok(SupportedType::U8),
            "vec_u16" => Ok(SupportedType::U16),
            "vec_u32" => Ok(SupportedType::U32),
            "vec_u64" => Ok(SupportedType::U64),
            "vec_u128" => Ok(SupportedType::U128),
            "vec_usize" => Ok(SupportedType::VecUsize),
            "vec_string" => Ok(SupportedType::VecString),
            "vec_whatever" => Ok(SupportedType::VecWhatever),
            //_ => Err("Provided type not recognised.")
            _ => Err(SendableText::Str("Provided value is not a suipported type."))

        }

    }

}

#[derive(Debug)]
pub enum TypeInstance
{

    Bool(bool),
    Char(char),

    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    U128(u128),
    USize(usize),

    //Collections

    String(String),

    Whatever(Whatever),

    //Vecs

    VecBool(Vec<bool>),
    VecChar(Vec<char>),

    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),

    VecI128(Vec<i128>),
    VecISize(Vec<isize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),

    VecU128(Vec<i128>),
    VecUSize(Vec<usize>),

    VecString(Vec<String>),
    VecWhatever(Vec<Whatever>),
    VecOptionWhatever(Vec<Option<Whatever>>),

}

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Default)]
pub struct Command
{

    id: Option<u32>,
    command: String, //Optional when namespaces get added.
    type_name: SupportedType,
    //namespace: Option<String>,
    params: Option<Vec<Option<TypeInstance>>>
    
}

#[derive(Debug)]
pub struct CommandError
{

    id: Option<u32>,
    message: SendableText,
    field: Option<&'static str>,
    index: Option<usize>,
    sub_index: Option<usize>,
    sub_index_2: Option<usize>

}

impl CommandError
{

    pub fn new(message: SendableText, id: Option<u32>, field: Option<&'static str>) -> Self
    {

        Self
        {

            id,
            message,
            field,
            index: None,
            sub_index: None,
            sub_index_2: None

        }

    }

    pub fn with_index(message: SendableText, id: Option<u32>, field: Option<&'static str>, index: usize) -> Self
    {

        Self
        {

            id,
            message,
            field: field,
            index: Some(index),
            sub_index: None,
            sub_index_2: None

        }

    }

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

//JSON

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

fn convert_number(number: Number, command: &Command, field: Option<&'static str>) -> Result<TypeInstance, CommandError>
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

                Err(CommandError::new(SendableText::Str("Unexpected error converting param to f64."), command.id, field))

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

                Err(CommandError::new(SendableText::Str("Unexpected error converting param to i64."), command.id, field))

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

                Err(CommandError::new(SendableText::Str("Unexpected error converting param to u64."), command.id, field))

            }

        }
        
    }
    else
    {

        Err(CommandError::new(SendableText::Str("Internal error converting param to unknown."), command.id, field))
        
    }

}

//Sub Vec/Array

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

fn process_map(map: Map<String, Value>, index_opt: Option<usize>, command: &Command, field: Option<&'static str>) -> Result<Vec<TypeInstance>, CommandError>
{

    if !map.is_empty()
    {

        let mut res_vec = Vec::with_capacity(map.len());

        let mut index: usize = 0;

        for (key, value) in map
        {

            match key.trim()
            {
    
                "type_bool" => 
                {

                    res_vec.push(into_bool(value, command.id, field, index_opt, Some(index))?);

                }
                "type_char" => 
                {

                    res_vec.push(into_char(value, command.id, field, index_opt, Some(index))?);

                }
                "type_f32" => 
                {



                }
                "type_f64" => 
                {

                    

                }
                "type_i8" => 
                {

                    

                }
                "type_i16" => 
                {

                    

                }
                "type_i32" => 
                {

                    

                }
                "type_i64" => 
                {

                    

                }
                "type_i128" => 
                {

                    

                }
                "type_isize" => 
                {

                    

                }
                "type_u8" => 
                {

                    

                }
                "type_u16" => 
                {

                    

                }
                "type_u32" => 
                {

                    

                }
                "type_u64" => 
                {

                    

                }
                "type_u128" => 
                {

                    

                }
                "type_usize" => 
                {

                    

                }
                "type_string" => 
                {

                    

                }
                "type_whatever" => 
                {

                    

                }
                "type_vec_bool" => 
                {

                    

                }
                "type_vec_char" => 
                {

                    

                }
                "type_vec_f32" => 
                {

                    

                }
                "type_vec_f64" => 
                {

                    

                }
                "type_vec_i8" => 
                {

                    

                }
                "type_vec_i16" => 
                {

                    

                }
                "type_vec_i32" => 
                {

                    

                }
                "type_vec_i64" => 
                {

                    

                }
                "type_vec_i128" => 
                {

                    

                }
                "type_vec_isize" => 
                {

                    

                }
                "type_vec_u8" => 
                {

                    

                }
                "type_vec_u16" => 
                {

                    

                }
                "type_vec_u32" => 
                {

                    

                }
                "type_vec_u64" => 
                {

                    

                }
                "type_vec_u128" => 
                {

                    

                }
                "type_vec_usize" => 
                {

                    

                }
                "type_vec_string" => 
                {

                    

                }
                "type_vec_whatever" => 
                {

                    

                }
                _ =>
                {

                    return Err(CommandError::with_index_opt(SendableText::Str("Internal error converting param at index to unknown."), command.id, field, index_opt));
    
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

        Ok(res_vec)

    }
    else
    {

        Err(CommandError::new(SendableText::Str("Empty Map not expected."), command.id, field))
        
    }

}

pub fn into_command(input: Value) -> Result<Command, CommandError>
{

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

                            return Err(CommandError::new(SendableText::Str("Integer expected in id field."), None, Some(field)));

                        }

                    }

                }
                else
                {

                    return Err(CommandError::new(SendableText::Str("Integer expected in id field."), None, Some(field)));                    
                    
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

                        return Err(CommandError::new(SendableText::Str("The command field must be a String."), command.id, Some(field)));  

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

                return Err(CommandError::new(SendableText::Str("Command field not found."), command.id, Some(field)));     
                
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

                                    command.type_name = st;

                                }
                                Err(err) =>
                                {

                                    return Err(CommandError::new(err, command.id, Some(field)));

                                    //return Err(SendableText::Str(err)); 

                                }

                            }

                        }
                        _ =>
                        {

                            return Err(CommandError::new(SendableText::Str("The command field must be a String."), command.id, Some(field)));  

                        }
                        
                    }

                }
                else
                {

                    return Err(CommandError::new(SendableText::Str("A type must be specified."), command.id, Some(field)));  
                    
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

                            let number = convert_number(val, &command, Some(field))?;

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

                                let mut index: usize = 0;

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

                                            match convert_number_from_vec(number, index, &command, Some(field))
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

                                            return Err(CommandError::new(SendableText::Str("Processing arrays without accompanying type information is not supported."), command.id, Some(field)));

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



                                            //return Err(CommandError::new(SendableText::String(format!("Map at param index: {}. Map params are not supported.", index)), command.id));

                                        }

                                    }
                                    
                                    index.pp();
                                    
                                }

                                command.params = Some(params_vec);

                            }

                        }
                        Value::Object(map) =>
                        {

                            //Multiple type annotated Values only.



                            //return Err(CommandError::new(SendableText::Str("The params field cannot be an Object."), command.id)); 

                        }

                    }


                    let parsed_params: Vec<Option<TypeInstance>> = Vec::new();



                }
                None =>
                {

                    //if let?

                }

            }

            error_message = "";

        }

    }

    //Ok(command)

    Err(CommandError::new(SendableText::Str(error_message), None, None))

}