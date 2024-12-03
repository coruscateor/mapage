//use axum::Error;

use corlib::text::SendableText;

use serde::{Deserialize, Serialize};

use crate::CommandError; //{impl_is_type_method, impl_is_type_methods, CommandError};

use super::{json::{CommandInterpretationError, Indices}, Whatever};

use paste::paste;

use std::{any::TypeId, convert::{From, TryFrom}, fmt::Debug}; //, path::Display};

use std::error::Error;

use std::fmt::Display;

//#[macro_export]
macro_rules! impl_is_type_method
{

    ($object_type:ty, $value_fn_name:ty, $enum_value_type:ty) =>
    {

        paste!
        {

            pub fn [<is_ $value_fn_name>](&self) -> bool
            {
    
                if let $object_type::$enum_value_type(_) = self
                {
        
                    true
                    
                }
                else
                {
        
                    false
                    
                }
               
            }

        }

    }

}

//#[macro_export]
macro_rules! impl_is_type_methods
{

    ($object_type:ty, $(($value_fn_name:ty, $enum_value_type:ty)),+) =>
    {

        $(

            impl_is_type_method!($object_type, $value_fn_name, $enum_value_type);

        )*

    }

}

macro_rules! impl_take_type_method
{

    ($value_fn_name:ty, $enum_value_type:ty) =>
    {

        paste!
        {

            pub fn [<take_ $value_fn_name>](self) -> Result<$value_fn_name, ()>
            {
    
                if let TypeInstance::$enum_value_type(val) = self
                {
        
                    Ok(val)
                    
                }
                else
                {
        
                    Err(())
                    
                }
               
            }

        }

    };
    ($value_fn_name:ty, $enum_value_type:ty, $return_type:ty) =>
    {

        paste!
        {

            pub fn [<take_ $value_fn_name>](self) -> Result<$return_type, ()>
            {
    
                if let TypeInstance::$enum_value_type(val) = self
                {
        
                    Ok(val)
                    
                }
                else
                {
        
                    Err(())
                    
                }
               
            }

        }

    }

}

macro_rules! impl_take_type_methods
{

    ($(($value_fn_name:ty, $enum_value_type:ty)),+) =>
    {

        $(

            impl_take_type_method!($value_fn_name, $enum_value_type);

        )*

    };
    /*
    ($(($value_fn_name:ty, $enum_value_type:ty, $return_type:ty)),+) =>
    {

        $(

            impl_take_type_method!($value_fn_name, $enum_value_type, $return_type);

        )*

    }
    */

}

macro_rules! impl_take_type_return_type_methods
{

    ($(($value_fn_name:ty, $enum_value_type:ty, $return_type:ty)),+) =>
    {

        $(

            impl_take_type_method!($value_fn_name, $enum_value_type, $return_type);

        )*

    }

}

#[derive(Debug, Serialize, Deserialize)]
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
    //Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    U128(u128),
    //Usize(usize),

    //Collections

    String(String),

    Whatever(Whatever),

    //Vecs

    VecBool(Vec<bool>),
    //VecChar(Vec<char>),

    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),

    VecI128(Vec<i128>),
    //VecISize(Vec<isize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),

    VecU128(Vec<u128>),
    //VecUSize(Vec<usize>),

    //VecString(Vec<String>),
    //VecWhatever(Vec<Whatever>),
    //VecOptionWhatever(Vec<Option<Whatever>>),

}

impl TypeInstance
{

    /*
    pub fn is_bool(&self) -> bool
    {

        if let TypeInstance::Bool(_) = self
        {

            true
            
        }
        else
        {

            false
            
        }

    }
    */

    //impl_is_type_method!(TypeInstance, char, Char);

    //impl_is_type_method!(TypeInstance, f32, F32);

    //impl_is_type_method!(TypeInstance, f64, F64);

    impl_is_type_methods!(TypeInstance,
    (bool, Bool),
    (char, Char),
    (f32, F32),
    (f64, F64),
    (i8, I8),
    (i16, I16),
    (i32, I32),
    (i64, I64),
    (i128, I128),
    (u8, U8),
    (u16, U16),
    (u32, U32),
    (u64, U64),
    (u128, U128),
    (string, String),
    (whatever, Whatever),
    (vec_bool, VecBool),
    (vec_f32, VecF32),
    (vec_f64, VecF64),
    (vec_i8, VecI8),
    (vec_i16, VecI16),
    (vec_i32, VecI32),
    (vec_i64, VecI64),
    (vec_i128, VecI128),
    (vec_u8, VecU8),
    (vec_u16, VecU16),
    (vec_u32, VecU32),
    (vec_u64, VecU64),
    (vec_u128, VecU128)
    );

    pub fn take_bool(self) -> Result<bool, ()> //TypeInstanceConversionError>
    {

        if let TypeInstance::Bool(val) = self
        {

            Ok(val)

        }
        else
        {

            Err(())

            //Err(TypeInstanceConversionError::default())
            
        }

    }

    impl_take_type_methods!(
    (char, Char),
    (f32, F32),
    (f64, F64),
    (i8, I8),
    (i16, I16),
    (i32, I32),
    (i64, I64),
    (i128, I128),
    (u8, U8),
    (u16, U16),
    (u32, U32),
    (u64, U64),
    (u128, U128));

    impl_take_type_return_type_methods!((string, String, String),
    //(whatever, Whatever, Whatever),
    (vec_bool, VecBool, Vec<bool>),
    (vec_f32, VecF32, Vec<f32>), //VecF32),
    (vec_f64, VecF64, Vec<f64>), //VecF64),
    (vec_i8, VecI8, Vec<i8>), //VecI8),
    (vec_i16, VecI16, Vec<i16>), //VecI16),
    (vec_i32, VecI32, Vec<i32>), //VecI32),
    (vec_i64, VecI64, Vec<i64>), //VecI64),
    (vec_i128, VecI128, Vec<i128>), //VecI128),
    (vec_u8, VecU8, Vec<u8>), //VecU8),
    (vec_u16, VecU16, Vec<u16>), //VecU16),
    (vec_u32, VecU32, Vec<u32>), //VecU32),
    (vec_u64, VecU64, Vec<u64>), //VecU64),
    (vec_u128, VecU128, Vec<u128>));  //VecU128));

    pub fn into_whatever(self) -> Result<Whatever, ()> //command_id: Option<u32>, field: Option<&'static str>, indices: &Option<Indices>) -> Result<Whatever, CommandInterpretationError> //, index: Option<usize>, sub_index: Option<usize>
    {

        match self
        {

            TypeInstance::Bool(val) => Ok(Whatever::Bool(val)),
            TypeInstance::Char(val) => Ok(Whatever::Char(val)),
            TypeInstance::F32(val)  => Ok(Whatever::F32(val)),
            TypeInstance::F64(val) => Ok(Whatever::F64(val)),
            TypeInstance::I8(val) => Ok(Whatever::I8(val)),
            TypeInstance::I16(val) => Ok(Whatever::I16(val)),
            TypeInstance::I32(val) => Ok(Whatever::I32(val)),
            TypeInstance::I64(val) => Ok(Whatever::I64(val)),
            TypeInstance::I128(val) => Ok(Whatever::I128(val)),
            //TypeInstance::Isize(val) => Ok(Whatever::Isize(val)),
            TypeInstance::U8(val) => Ok(Whatever::U8(val)),
            TypeInstance::U16(val) => Ok(Whatever::U16(val)),
            TypeInstance::U32(val) => Ok(Whatever::U32(val)),
            TypeInstance::U64(val) => Ok(Whatever::U64(val)),
            TypeInstance::U128(val) => Ok(Whatever::U128(val)),
            //TypeInstance::Usize(val) => Ok(Whatever::USize(val)),
            TypeInstance::String(val) => Ok(Whatever::String(val)),
            TypeInstance::Whatever(val) => Ok(val),
            TypeInstance::VecBool(vec) => Ok(Whatever::VecBool(vec)),
            //TypeInstance::VecChar(vec) => Ok(Whatever::VecChar(vec)),
            TypeInstance::VecF32(vec) => Ok(Whatever::VecF32(vec)),
            TypeInstance::VecF64(vec) => Ok(Whatever::VecF64(vec)),
            TypeInstance::VecI8(vec) => Ok(Whatever::VecI8(vec)),
            TypeInstance::VecI16(vec) => Ok(Whatever::VecI16(vec) ),
            TypeInstance::VecI32(vec) => Ok(Whatever::VecI32(vec)),
            TypeInstance::VecI64(vec) => Ok(Whatever::VecI64(vec)),
            TypeInstance::VecI128(vec) => Ok(Whatever::VecI128(vec)),
            //TypeInstance::VecISize(vec) => Ok(Whatever::VecISize(vec) ),
            TypeInstance::VecU8(vec) => Ok(Whatever::VecU8(vec)),
            TypeInstance::VecU16(vec) => Ok(Whatever::VecU16(vec)),
            TypeInstance::VecU32(vec) => Ok(Whatever::VecU32(vec)),
            TypeInstance::VecU64(vec) => Ok(Whatever::VecU64(vec)),
            TypeInstance::VecU128(vec) => Ok(Whatever::VecU128(vec) ),
            //TypeInstance::VecUSize(vec) => Ok(Whatever::VecUSize(vec)),
            //TypeInstance::VecString(vec) => Ok(Whatever::VecString(vec)),
            _ =>
            {

                Err(())

                //Err(CommandInterpretationError::new(SendableText::Str("Conversion Error"), command_id, field, indices.clone())) //with_sub_index_opt(SendableText::Str("Conversion Error"), command_id, field, index, sub_index))

            }

        }

    }

    pub fn get_sendable_type_name(&self) -> SendableText
    {

        match self
        {

            TypeInstance::Bool(_) => SendableText::Str("bool"),
            TypeInstance::Char(_) => SendableText::Str("char"),
            TypeInstance::F32(_) => SendableText::Str("f32"),
            TypeInstance::F64(_) => SendableText::Str("f64"),
            TypeInstance::I8(_) => SendableText::Str("i8"),
            TypeInstance::I16(_) => SendableText::Str("i16"),
            TypeInstance::I32(_) => SendableText::Str("i32"),
            TypeInstance::I64(_) => SendableText::Str("i64"),
            TypeInstance::I128(_) => SendableText::Str("i128"),
            TypeInstance::U8(_) => SendableText::Str("u8"),
            TypeInstance::U16(_) => SendableText::Str("u16"),
            TypeInstance::U32(_) => SendableText::Str("u32"),
            TypeInstance::U64(_) => SendableText::Str("u64"),
            TypeInstance::U128(_) => SendableText::Str("u128"),
            TypeInstance::String(_) => SendableText::Str("string"),
            TypeInstance::Whatever(_whatever) => SendableText::Str("whatever"),
            TypeInstance::VecBool(_vec) => SendableText::Str("vec_bool"),
            TypeInstance::VecF32(_vec) => SendableText::Str("vec_f32"),
            TypeInstance::VecF64(_vec) => SendableText::Str("vec_f64"),
            TypeInstance::VecI8(_vec) => SendableText::Str("vec_i8"),
            TypeInstance::VecI16(_vec) => SendableText::Str("vec_i16"),
            TypeInstance::VecI32(_vec) => SendableText::Str("vec_i32"),
            TypeInstance::VecI64(_vec) => SendableText::Str("vec_i64"),
            TypeInstance::VecI128(_vec) => SendableText::Str("vec_i128"),
            TypeInstance::VecU8(_vec) => SendableText::Str("vec_u8"),
            TypeInstance::VecU16(_vec) => SendableText::Str("vec_u16"),
            TypeInstance::VecU32(_vec) => SendableText::Str("vec_u32"),
            TypeInstance::VecU64(_vec) => SendableText::Str("vec_u64"),
            TypeInstance::VecU128(_vec) => SendableText::Str("vec_u128")

        }

    }

}

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

            panic!("Error: Can't do it.")
            
        }
        
    }

}

/*
impl From<Option<TypeInstance>> for Option<bool>
{

    fn from(value: Option<TypeInstance>) -> Self
    {

        if let Some(ti) = value
        {

            if let TypeInstance::Bool(val) = ti //.expect("Error: Value has no value.")
            {
    
                Some(val)
    
            }
            else
            {
    
                panic!("Error: Can't do it.")
                
            }

        }
        else
        {

            None
            
        }
        
    }

}
*/

/*
#[macro_export]
macro_rules! from_type_instance_type
{

    ($lc_ti_type:ident, $ti_type:ident) =>
    {

        impl From<TypeInstance> for $lc_ti_type
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
*/

pub struct TypeInstanceConversionError
{

    message: SendableText

}

impl TypeInstanceConversionError
{

    pub fn new(message: SendableText) -> Self
    {

        Self
        {

            message

        }

    }

    pub fn new_str(message: &'static str) -> Self
    {

        Self
        {

            message: SendableText::Str(message)

        }

    }

    pub fn take(self) -> SendableText
    {

        self.message

    }

}

impl Default for TypeInstanceConversionError
{

    fn default() -> Self
    {
        Self
        {
            
            message: SendableText::Str("Invalid conversion")
        
        }

    }

}

impl Display for TypeInstanceConversionError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        f.write_str(&self.message)
        
    }

}

impl Debug for TypeInstanceConversionError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        f.debug_struct("TypeInstanceConversionError").field("message", &self.message).finish()

    }

}

impl Error for TypeInstanceConversionError
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        
        self.source()

    }

    //fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}

/*
impl<T> TryFrom<T> for TypeInstance
{
    
    type Error = TypeInstanceConversionError;

    fn try_from(_value: T) -> Result<Self, Self::Error>
    {

        Err(TypeInstanceConversionError::new_str("Invalid conversion"))

        /*
        let t_id = TypeId::of::<T>();

        match t_id
        {

            TypeId::of::<bool>() =>
            {



            } 

        }
        */

        /* 
        if let TypeInstance::Bool(val) = value
        {

            Ok(val)
            
        }
        else
        {

            Err(TypeInstanceConversionError::new_str("Bool expected"))

            //panic!("Error: Invalid conversion");
            
        }
        */
       
    }

}


impl TryFrom<bool> for TypeInstance
{
    
    type Error = TypeInstanceConversionError;

    fn try_from(value: bool) -> Result<Self, Self::Error>
    {

        Ok(TypeInstance::Bool(value))

    }

}
*/

/*
#[macro_export]
macro_rules! try_from_type_instance_type
{

    ($impl_for:ident, $ti_type:ident) =>
    {

        impl TryFrom<TypeInstance> for $impl_for
        {
            
            type Error = TypeInstanceConversionError;

            fn try_from(value: TypeInstance) -> Result<Self, Self::Error>
            {

                if let TypeInstance:: $ti_type (val) = value
                {

                    Ok(val)
                    
                }
                else
                {

                    Err(TypeInstanceConversionError::new_str(concat!(stringify!($ti_type), " expected")))
                    
                }
            
            }

        }

    }

}

impl TryFrom<TypeInstance> for bool
{
    
    type Error = TypeInstanceConversionError;

    fn try_from(value: TypeInstance) -> Result<Self, Self::Error>
    {

        if let TypeInstance::Bool(val) = value
        {

            Ok(val)
            
        }
        else
        {

            Err(TypeInstanceConversionError::new_str("Bool expected"))

            //panic!("Error: Invalid conversion");
            
        }
       
    }

}

try_from_type_instance_type!(char, Char);

try_from_type_instance_type!(f32, F32);

try_from_type_instance_type!(f64, F64);

try_from_type_instance_type!(i8, I8);

try_from_type_instance_type!(i16, I16);

try_from_type_instance_type!(i32, I32);

try_from_type_instance_type!(i64, I64);

try_from_type_instance_type!(i128, I128);

try_from_type_instance_type!(u8, U8);

try_from_type_instance_type!(u16, U16);

try_from_type_instance_type!(u32, U32);

try_from_type_instance_type!(u64, U64);

try_from_type_instance_type!(u128, U128);

try_from_type_instance_type!(String, String);

try_from_type_instance_type!(Whatever, Whatever);

pub type VecBool = Vec<bool>;

try_from_type_instance_type!(VecBool, VecBool);

pub type VecF32 = Vec<f32>;

try_from_type_instance_type!(VecF32, VecF32);

pub type VecF64 = Vec<f64>;

try_from_type_instance_type!(VecF64, VecF64);

pub type VecI8 = Vec<i8>;

try_from_type_instance_type!(VecI8, VecI8);

pub type VecI16 = Vec<i16>;

try_from_type_instance_type!(VecI16, VecI16);

pub type VecI32 = Vec<i32>;

try_from_type_instance_type!(VecI32, VecI32);

pub type VecI64 = Vec<i64>;

try_from_type_instance_type!(VecI64, VecI64);

pub type VecI128 = Vec<i128>;

try_from_type_instance_type!(VecI128, VecI128);

pub type VecU8 = Vec<u8>;

try_from_type_instance_type!(VecU8, VecU8);

pub type VecU16 = Vec<u16>;

try_from_type_instance_type!(VecU16, VecU16);

pub type VecU32 = Vec<u32>;

try_from_type_instance_type!(VecU32, VecU32);

pub type VecU64 = Vec<u64>;

try_from_type_instance_type!(VecU64, VecU64);

pub type VecU128 = Vec<u128>;

try_from_type_instance_type!(VecU128, VecU128);
*/

/*
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
*/

/*
#[macro_export]
macro_rules! try_from_type_instance_type_uc
{

    ($impl_for:ty, $ti_type:ident) =>
    {

        impl TryFrom<TypeInstance> for $impl_for
        {
            
            type Error = TypeInstanceConversionError;

            fn try_from(value: TypeInstance) -> Result<Self, Self::Error>
            {

                if let TypeInstance:: $ti_type (val) = value
                {

                    Ok(val)
                    
                }
                else
                {

                    Err(TypeInstanceConversionError::new_str("$ti_type expected"))
                    
                }
            
            }

        }

    }

}

try_from_type_instance_type_uc!(String);

try_from_type_instance_type_uc!(Whatever);
*/

