use std::string::ToString;
use async_graphql::{SimpleObject, InputObject};

use std::sync::Arc;

use crate::stored_object::StoredObject;

use crate::stored_objects::bool::Bool;

use crate::generic_type_info::*;

///
/// Rcd (Reference Counted) is an Arc in this build
/// 
pub type Rcd<T> = Arc<T>;

///
/// Reference counted type
///
//pub type RcdType = Rcd<Type>;

//Type names

pub const BOOL: &str = "bool";

pub const CHAR: &str = "char";

pub const F32: &str = "f32";

pub const F64: &str = "f64";

pub const I8: &str = "i8";

pub const I16: &str = "i16";

pub const I32: &str = "i32";

pub const I64: &str = "i64";

pub const I128: &str = "i128";

pub const ISIZE: &str = "isize";

pub const U8: &str = "u8";

pub const U16: &str = "u16";

pub const U32: &str = "u32";

pub const U64: &str = "u64";

pub const U128: &str = "u128";

pub const UNIT: &str = "unit";

pub const USIZE: &str = "usize";

//

pub const NOTHING: &str = "Nothing";

//pub const BOOL: &str = "Optional";

//pub const BOOL: &str = "Required";

//pub const BOOL: &str = "Something";

pub const WHATEVER: &str = "Whatever";

//

pub trait Type
{

    fn get_name(&self) -> &str;

    fn is_generic(&self) -> bool;

}

#[derive(Clone, InputObject, SimpleObject)]
pub struct NonGenericType
{

    name: &'static str

}

impl NonGenericType
{

    pub fn new(name: &str) -> Self //(name: String) -> Self
    {

        Self
        {

            name

        }

    }

    pub fn new_bool() -> Self
    {
        
        NonGenericType::new(BOOL)

    }

    pub fn new_char() -> Self
    {
        
        NonGenericType::new(CHAR)

    }

    pub fn new_f32() -> Self
    {
        
        NonGenericType::new(F32)

    }

    pub fn new_f64() -> Self
    {
        
        NonGenericType::new(F64)

    }

    pub fn new_i8() -> Self
    {
        
        NonGenericType::new(I8)

    }

    pub fn new_i16() -> Self
    {
        
        NonGenericType::new(I16)

    }

    pub fn new_i32() -> Self
    {
        
        NonGenericType::new(I32)

    }

    pub fn new_i64() -> Self
    {
        
        NonGenericType::new(I64)

    }

    pub fn new_i128() -> Self
    {
        
        NonGenericType::new(I128)

    }

    pub fn new_isize() -> Self
    {
        
        NonGenericType::new(ISIZE)

    }

    pub fn new_u8() -> Self
    {
        
        NonGenericType::new(U8)

    }

    pub fn new_u16() -> Self
    {
        
        NonGenericType::new(U16)

    }

    pub fn new_u32() -> Self
    {
        
        NonGenericType::new(U32)

    }

    pub fn new_u64() -> Self
    {
        
        NonGenericType::new(U64)

    }

    pub fn new_u128() -> Self
    {
        
        NonGenericType::new(U128)

    }

    pub fn new_unit() -> Self
    {
        
        NonGenericType::new(UNIT)

    }

    pub fn new_usize() -> Self
    {
        
        NonGenericType::new(USIZE)

    }

    //

    pub fn new_nothing() -> Self
    {
        
        NonGenericType::new(NOTHING)

    }

    /*
    pub fn new_optional() -> Self
    {
        
        NonGenericType::new("Optional")

    }

    pub fn new_required() -> Self
    {
        
        NonGenericType::new("Required")

    }

    pub fn new_something() -> Self
    {

        NonGenericType::new("Something")

    }
    */
    
    pub fn new_whatever() -> Self
    {
        
        NonGenericType::new("Whatever")

    }

    //Types - vector



    //

    pub fn get_default_value(&self) -> std::result::Result<Box<dyn StoredObject>, &'static str>
    {

        //typ: Rcd<Type>

        match self.name //.as_str()
        {

            "bool" => {

                return Ok(Box::new(Bool::default()));

            }
            _ => {

            }

        }

        //Is generic?

        //Nothing matches

        Err("No matching types")

    }


}

impl Type for NonGenericType
{

    fn get_name(&self) -> &str
    {

        self.name.as_str()

    }

    fn is_generic(&self) -> bool
    {

        false

    }

}

impl ToString for NonGenericType
{

    fn to_string(&self) -> String {

        self.name.to_string()

    }

}

impl PartialEq for NonGenericType
{

    fn eq(&self, other: &Self) -> bool
    {

        self.name == other.name

    }

}

impl Eq for NonGenericType {}

#[derive(Clone, InputObject, SimpleObject)]
pub struct GenericType
{

    //name: &'static str,
    generic_type_info: Rcd<GenericTypeInfo>,
    parameters: Rcd<Vec<Rcd<dyn Type>>>,
    //is_variadic: bool

}

impl GenericType
{

    pub fn new(generic_type_info: Rcd<GenericTypeInfo>, parameters: Rcd<Vec<Rcd<dyn Type>>>) -> Self //(name: String) -> Self
    {

        //check parameters are correct


        Self
        {

            generic_type_info,
            parameters //: Vec::new(),
            //is_variadic: false

        }

    }

    /*
    pub fn new_rcd(name: &str) -> Self //(name: String) -> Self
    {

        Self
        {

            name: name.to_string(),
            parameters: Vec::new(),
            is_variadic: false

        }

    }
    */

    /*
    pub fn new_generic_single(generic_type_info: Rcd<GenericTypeInfo>, typ: &Rcd<dyn Type>) -> Self
    {

        
        Self
        {

            generic_type_info,
            parameters: vec![typ.clone()],
            //is_variadic: false

        }


    }

    pub fn new_generic_multi(name: &str, types: Vec<dyn Type>) -> Self //std::slice::Iter<Type>) -> Self
    {

        //let mut t_vec = Vec::with_capacity(5);

        //t_vec.clone_from_slice(types);

        Self
        {

            generic_type_info,
            parameters: types, //t_vec,
            //is_variadic: false

        }

    }
    */

    ///
    /// Is variadic generic
    ///
    /*
    pub fn new_variadic(name: &str) -> Self
    {

        Self
        {

            name: name.to_string(),
            parameters: Vec::new(),
            //is_variadic: true

        }

    }
    */

    /* 
    pub const fn const_new(name: &str) -> Self
    {

        Self
        {

            name: name.to_string(),
            parameters: Vec::new(),
            is_variadic: false

        }

    }
    */

    //Types - scalar

    /*
    pub fn new_bool() -> Self
    {
        
        Type::new("bool")

    }

    pub fn new_char() -> Self
    {
        
        Type::new("char")

    }

    pub fn new_f32() -> Self
    {
        
        Type::new("f32")

    }

    pub fn new_f64() -> Self
    {
        
        Type::new("f64")

    }

    pub fn new_i8() -> Self
    {
        
        Type::new("i8")

    }

    pub fn new_i16() -> Self
    {
        
        Type::new("i16")

    }

    pub fn new_i32() -> Self
    {
        
        Type::new("i32")

    }

    pub fn new_i64() -> Self
    {
        
        Type::new("i64")

    }

    pub fn new_i128() -> Self
    {
        
        Type::new("i128")

    }

    pub fn new_isize() -> Self
    {
        
        Type::new("isize")

    }

    pub fn new_u8() -> Self
    {
        
        Type::new("u8")

    }

    pub fn new_u16() -> Self
    {
        
        Type::new("u16")

    }

    pub fn new_u32() -> Self
    {
        
        Type::new("u32")

    }

    pub fn new_u64() -> Self
    {
        
        Type::new("u64")

    }

    pub fn new_u128() -> Self
    {
        
        Type::new("u128")

    }

    pub fn new_unit() -> Self
    {
        
        Type::new("unit")

    }

    pub fn new_usize() -> Self
    {
        
        Type::new("usize")

    }

    //

    pub fn new_nothing() -> Self
    {
        
        Type::new("Nothing")

    }

    pub fn new_optional() -> Self
    {
        
        Type::new("Optional")

    }

    pub fn new_required() -> Self
    {
        
        Type::new("Required")

    }

    pub fn new_something() -> Self
    {

        Type::new("Something")

    }
    
    pub fn new_whatever() -> Self
    {
        
        Type::new("Whatever")

    }
    */

    //Types - vector



    //

    /*
    pub fn get_name(&self) -> &str
    {

        self.generic_type_info.get_name() //.name.as_str()

    }
    */

    pub fn get_generic_type_info_ref(&self) -> &GenericTypeInfo
    {

        self.generic_type_info.as_ref()

    }

    pub fn get_generic_type_info(&self) -> Rcd<GenericTypeInfo>
    {

        self.generic_type_info.clone()

    }

    pub fn get_parameters_ref(&self) -> &Vec<Rcd<dyn Type>>
    {

        self.parameters.as_ref()

    }

    pub fn get_parameters(&self) -> Rcd<Vec<Rcd<dyn Type>>>
    {

        self.parameters.clone()

    }

    pub fn get_parameters_iter(&self) -> std::slice::Iter<Rcd<Type>>
    {

        self.parameters.iter()

    }

    /*
    pub fn is_generic(&self) -> bool
    {

        //self.parameters.is_empty() || self.is_variadic

        true

    }
    
    pub fn get_parameters_len(&self) -> usize
    {

        self.parameters.len()

    }
    */

    /*
    pub fn get_is_variadic(&self) -> bool
    {

        self.is_variadic

    }

    pub fn get_default_value(typ: Type) -> std::result::Result<Box<dyn StoredObject>, &'static str>
    {

        match typ.name.as_str()
        {

            "bool" => {

                return Ok(Box::new(Bool::default()));

            }
            _ => {

            }

        }

        //Is generic?

        //Nothing matches

        Err("No matching types")

    }
    */

}

impl Type for GenericType
{

    fn get_name(&self) -> &str
    {

        self.generic_type_info.get_name()

    }

    fn is_generic(&self) -> bool
    {

        true

    }

}

impl ToString for GenericType
{

    fn to_string(&self) -> String {

        let mut to_return = String::with_capacity(100);

        to_return.push_str(self.name.as_str());

        if !self.parameters.is_empty()
        {

            to_return.push('<');

            let mut count: usize = 0;

            for val in self.parameters.iter()
            {
    
                if count > 1
                {
    
                    to_return.push_str(", ");
    
                }
    
                to_return.push_str(val.to_string().as_str());
    
                count += 1;
    
            }

            to_return.push('>');

        }
        
        to_return

    }

}

impl PartialEq for GenericType
{

    fn eq(&self, other: &Self) -> bool
    {

        if self.name == other.name && self.is_variadic == other.is_variadic
        {

            let len = self.parameters.len();

            if len == other.parameters.len()
            {

                let mut index: usize = 0;

                while index < len
                {

                    if self.parameters[index] != other.parameters[index]
                    {

                        return false;

                    }

                    index += 1;

                }

            }

        }

        true

    }

}

impl Eq for GenericType {}


