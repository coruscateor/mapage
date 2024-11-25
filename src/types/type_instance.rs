use corlib::text::SendableText;

use serde::Serialize;

use crate::CommandError; //{impl_is_type_method, impl_is_type_methods, CommandError};

use super::{json::{CommandInterpretationError, Indices}, Whatever};

use paste::paste;

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

#[derive(Debug, Serialize)]
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

    VecU128(Vec<i128>),
    //VecUSize(Vec<usize>),

    //VecString(Vec<String>),
    //VecWhatever(Vec<Whatever>),
    //VecOptionWhatever(Vec<Option<Whatever>>),

}

impl TypeInstance
{

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

    //impl_is_type_method!(TypeInstance, char, Char);

    //impl_is_type_method!(TypeInstance, f32, F32);

    //impl_is_type_method!(TypeInstance, f64, F64);

    impl_is_type_methods!(TypeInstance, (char, Char),
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

    pub fn into_whatever(self, command_id: Option<u32>, field: Option<&'static str>, indices: &Option<Indices>) -> Result<Whatever, CommandInterpretationError> //, index: Option<usize>, sub_index: Option<usize>
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

                Err(CommandInterpretationError::new(SendableText::Str("Conversion Error"), command_id, field, indices.clone())) //with_sub_index_opt(SendableText::Str("Conversion Error"), command_id, field, index, sub_index))

            }

        }

    }

}

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


