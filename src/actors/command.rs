use std::{default, string};

//use anyhow::Ok;

use serde::{Deserialize, Serialize};

use serde_json::Value;

use corlib::text::SendableText;

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

    VecString

}

impl SupportedType
{

    pub fn try_parse(slice: &str) -> Result<SupportedType, ()> //&str>
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
            //_ => Err("Provided type not recognised.")
            _ => Err(())
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

    VecString(Vec<String>)

}

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Default)]
pub struct Command
{

    id: Option<u32>,
    command: String, //Optional when namespaces get added.
    supported_type: SupportedType,
    //namespace: Option<String>,
    params: Option<Vec<Option<TypeInstance>>>
    
}

//JSON

pub fn into_command(input: Value) -> Result<Command, SendableText>
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

                            return Err(SendableText::Str("Integer expected for id field."));

                        }

                    }

                }
                else
                {

                    return Err(SendableText::Str("Number expected in id field."));                    
                    
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

                        return Err(SendableText::Str("The command field must be a String."));  

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

                return Err(SendableText::Str("Command field not found."));     
                
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

                                    command.supported_type = st;

                                }
                                Err(err) =>
                                {

                                    //return Err(SendableText::Str(err)); 

                                }

                            }

                        }
                        _ =>
                        {

                            return Err(SendableText::Str("The command field must be a String."));  

                        }
                        
                    }


                }
                else
                {

                    return Err(SendableText::Str("A type must be specified."));  
                    
                }

            }

            //Params array...

            error_message = "";

        }

    }

    Err(SendableText::Str(error_message))

}