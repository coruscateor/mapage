use std::fmt::Display;
use std::{default, string};

//use anyhow::Ok;

//use anyhow::Error;

use std::error::Error;

use serde::{Deserialize, Serialize};

use serde_json::{Number, Value};

use corlib::text::SendableText;

use corlib::inc_dec::{self, IncrementsSelf};

use crate::types::Whatever;

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
    message: SendableText

}

impl CommandError
{

    pub fn new(message: SendableText, id: Option<u32>) -> Self
    {

        Self
        {

            id,
            message

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

fn convert_number_from_vec(number: Number, index: usize, command: &Command) -> Result<TypeInstance, CommandError>
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

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to f64.", index)), command.id))

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

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to i64.", index)), command.id))

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

                Err(CommandError::new(SendableText::String(format!("Unexpected error converting param at index: {} to u64.", index)), command.id))

            }

        }
        
    }
    else
    {

        Err(CommandError::new(SendableText::String(format!("Internal error converting param at index: {} to unknown.", index)), command.id))
        
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

            let id_opt = map.remove("id");

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

                            return Err(CommandError::new(SendableText::Str("Integer expected in id field."), None));

                        }

                    }

                }
                else
                {

                    return Err(CommandError::new(SendableText::Str("Integer expected in id field."), None));                    
                    
                }

            }

            //let command_opt = map.get("command");

            let command_opt = map.remove("command");

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

                        return Err(CommandError::new(SendableText::Str("The command field must be a String."), command.id));  

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

                return Err(CommandError::new(SendableText::Str("Command field not found."), command.id));     
                
            }

            let type_opt = map.remove("type");

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

                                    return Err(CommandError::new(err, command.id));

                                    //return Err(SendableText::Str(err)); 

                                }

                            }

                        }
                        _ =>
                        {

                            return Err(CommandError::new(SendableText::Str("The command field must be a String."), command.id));  

                        }
                        
                    }


                }
                else
                {

                    return Err(CommandError::new(SendableText::Str("A type must be specified."), command.id));  
                    
                }

            }

            //Params array...

            let params_opt = map.remove("params");

            match params_opt
            {

                Some(params) =>
                {

                    match params
                    {

                        Value::Null =>
                        {

                            command.params = None;

                        }
                        Value::Bool(_) =>
                        {

                            return Err(CommandError::new(SendableText::Str("The params field cannot be a bool value."), command.id));  

                        }
                        Value::Number(_) =>
                        {

                            return Err(CommandError::new(SendableText::Str("The params field cannot be a number value."), command.id));  

                        }
                        Value::String(_) =>
                        {

                            return Err(CommandError::new(SendableText::Str("The params field cannot be a String."), command.id));  

                        }
                        Value::Array(vec) =>
                        {

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

                                            match convert_number_from_vec(number, index, &command)
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
                                        Value::Array(vec_val) =>
                                        {

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

                                                        vec_param.push(Some(TypeInstance::Bool(val)));

                                                    }
                                                    Value::Number(number) =>
                                                    {

                                                        match convert_number_from_vec(number, index, &command)
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

                                                        vec_param.push(Some(TypeInstance::String(val)));

                                                    }
                                                    Value::Array(_vec) =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Array at param index: {}, sub-index: {}. Arrays of arrays are not supported.", index, sub_index)), command.id));

                                                    }
                                                    Value::Object(_map) =>
                                                    {

                                                        return Err(CommandError::new(SendableText::String(format!("Map at param index: {}, sub-index: {}. Map params are not supported.", index, sub_index)), command.id));

                                                    }

                                                }

                                                sub_index.pp();

                                            }

                                            params_vec.push(Some(TypeInstance))

                                        }
                                        Value::Object(_map) =>
                                        {

                                            return Err(CommandError::new(SendableText::String(format!("Map at param index: {}. Map params are not supported.", index)), command.id));

                                        }

                                    }
                                    
                                    index.pp();
                                    
                                }

                                command.params = Some(params_vec);

                            }

                        }
                        Value::Object(map) =>
                        {

                            return Err(CommandError::new(SendableText::Str("The params field cannot be an Object."), command.id)); 

                        }

                    }


                    let parsed_params: Vec<Option<TypeInstance>> = Vec::new();



                }
                None =>
                {



                }

            }

            error_message = "";

        }

    }

    //Ok(command)

    Err(CommandError::new(SendableText::Str(error_message), None))

}