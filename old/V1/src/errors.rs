use std::{num::{ParseIntError, ParseFloatError}, str::FromStr};

use async_graphql::{Result, Error, ErrorExtensions};

static INVALID_OPERTATION: &str = "Invalid Opertation";

pub fn invalid_operation<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(INVALID_OPERTATION))

}

static INVALID_OPERTATION_INSTANCE_IS_IMMUTABLE: &str = "Invalid Opertation: Instance is immuatable";

pub fn invalid_operation_instance_is_immuatable<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(INVALID_OPERTATION_INSTANCE_IS_IMMUTABLE))

}

//Stored Objects and Namespace

static STORED_OBJECT_ALREADY_EXISTS_IN_SPECIFICED_NAMESPACE: &str = "Stored Object already exists in specified namespace";  

pub fn stored_object_already_exists_in_specified_namespace<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(STORED_OBJECT_ALREADY_EXISTS_IN_SPECIFICED_NAMESPACE))

}

static TYPE_CONVERSION_ERROR: &str = "Type Conversion Errpr";  

pub fn type_conversion_error<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(TYPE_CONVERSION_ERROR))

}

/*
only traits defined in the current crate can be implemented for types defined outside of the crate
define and implement a trait or new type insteadrustcE0117
errors.rs(43, 30): `async_graphql::Error` is not defined in the current crate
errors.rs(43, 6): `ParseIntError` is not defined in the current crate

impl From<ParseIntError> for async_graphql::Error
{
    fn from(_: ParseIntError) -> Self {
        todo!()
    }
}
*/

pub fn parse_int<T>(val: &String) -> async_graphql::Result<T> //, ET> //async_graphql::Error
    where T: FromStr<Err = ParseIntError> //ET> //ParseIntError>
{

    let res = val.parse::<T>();

    match res
    {

        Ok(nval) => Ok(nval),
        Err(err) => get_parse_int_error(err)

    }

    /*
    if let Err(err) = res
    {



    }
    */

}

pub fn get_parse_int_error<T>(err: ParseIntError) -> async_graphql::Result<T> //async_graphql::Error
{

    let er_kind;

    match err.kind()
    {
        std::num::IntErrorKind::Empty => er_kind = "Empty",
        std::num::IntErrorKind::InvalidDigit => er_kind = "InvalidDigit",
        std::num::IntErrorKind::PosOverflow => er_kind = "PosOverflow",
        std::num::IntErrorKind::NegOverflow => er_kind = "NegOverflow",
        std::num::IntErrorKind::Zero => er_kind = "Zero",
        _ => er_kind = "_",
    }

    Err(err.extend_with(|error, e| e.set("IntErrorKind", er_kind)))

    //let mut aerr = async_graphql::Error::new(TYPE_CONVERSION_ERROR)

    //aerr.m

}

pub fn parse_float<T>(val: &String) -> async_graphql::Result<T> //, ET> //async_graphql::Error
    where T: FromStr<Err = ParseFloatError> //ET> //ParseIntError>
{

    let res = val.parse::<T>();

    match res
    {

        Ok(nval) => Ok(nval),
        Err(err) => get_parse_float_error(err)

    }

}

pub fn get_parse_float_error<T>(err: ParseFloatError) -> async_graphql::Result<T> 
{

    Err(err.extend_with(|error, e| {}))

}

static STORAGE_NOT_FOUND: &str = "No storage with the provided name and namespace exists";  

pub fn storage_not_found_error<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(STORAGE_NOT_FOUND))

}

static STORED_OPTION_IS_NONE: &str = "Option is None";  

pub fn stored_option_is_none_error<T>() -> async_graphql::Result<T>
{
    
    Err(Error::new(STORED_OPTION_IS_NONE))

}

static NAMESPACE_ALREADY_EXISTS: &str = "Namespace already exists";  

pub fn Namespace_already_exists<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(NAMESPACE_ALREADY_EXISTS))

}

static NO_NAMESPACE_FOUND_WITH_PROVIDED_NAME: &str = "No namespace found with provided name";  

pub fn no_namespace_found_with_provided_name<T>() -> async_graphql::Result<T>
{
        
    Err(Error::new(NO_NAMESPACE_FOUND_WITH_PROVIDED_NAME))

}

static FEATURE_NOT_IMPLEMETED: &str = "feature not implemented";  

pub fn feature_not_implemented<T>() -> async_graphql::Result<T>
{

    Err(Error::new(FEATURE_NOT_IMPLEMETED))

}


