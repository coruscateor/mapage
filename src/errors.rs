use std::{num::{ParseIntError, ParseFloatError}, str::FromStr};

use anyhow::{Result, Error};

static INVALID_OPERTATION: &str = "Invalid Opertation";

pub fn invalid_operation<T>() -> Result<T>
{
        
    Err(Error::msg(INVALID_OPERTATION))

}

static INVALID_OPERTATION_INSTANCE_IS_IMMUTABLE: &str = "Invalid Opertation: Instance is immuatable";

pub fn invalid_operation_instance_is_immuatable<T>() -> Result<T>
{
        
    Err(Error::msg(INVALID_OPERTATION_INSTANCE_IS_IMMUTABLE))

}

//Stored Objects and Namespace

static STORED_OBJECT_ALREADY_EXISTS_IN_SPECIFICED_NAMESPACE: &str = "Stored Object already exists in specified namespace";  

pub fn stored_object_already_exists_in_specified_namespace<T>() -> Result<T>
{
        
    Err(Error::msg(STORED_OBJECT_ALREADY_EXISTS_IN_SPECIFICED_NAMESPACE))

}

static TYPE_CONVERSION_ERROR: &str = "Type Conversion Errpr";  

pub fn type_conversion_error<T>() -> Result<T>
{
        
    Err(Error::msg(TYPE_CONVERSION_ERROR))

}

pub fn parse_int<T>(val: &String) -> Result<T>
    where T: FromStr<Err = ParseIntError>
{

    let res = val.parse::<T>();

    match res
    {

        Ok(nval) => Ok(nval),
        Err(err) => get_parse_int_error(err)

    }

}

pub fn get_parse_int_error<T>(err: ParseIntError) -> Result<T>
{

    Result::Err(Error::new(err))

}

pub fn parse_float<T>(val: &String) -> Result<T>
    where T: FromStr<Err = ParseFloatError>
{

    let res = val.parse::<T>();

    match res
    {

        Ok(nval) => Ok(nval),
        Err(err) => get_parse_float_error(err)

    }

}

pub fn get_parse_float_error<T>(err: ParseFloatError) -> Result<T> 
{

    Result::Err(Error::new(err))

}

static STORAGE_NOT_FOUND: &str = "No storage with the provided name and namespace exists";  

pub fn storage_not_found_error<T>() -> Result<T>
{
        
    Err(Error::msg(STORAGE_NOT_FOUND))

}

static STORED_OPTION_IS_NONE: &str = "Option is None";  

pub fn stored_option_is_none_error<T>() -> Result<T>
{
    
    Err(Error::msg(STORED_OPTION_IS_NONE))

}

static NAMESPACE_ALREADY_EXISTS: &str = "Namespace already exists";  

pub fn Namespace_already_exists<T>() -> Result<T>
{
        
    Err(Error::msg(NAMESPACE_ALREADY_EXISTS))

}

static NO_NAMESPACE_FOUND_WITH_PROVIDED_NAME: &str = "No namespace found with provided name";  

pub fn no_namespace_found_with_provided_name<T>() -> Result<T>
{
        
    Err(Error::msg(NO_NAMESPACE_FOUND_WITH_PROVIDED_NAME))

}

static FEATURE_NOT_IMPLEMETED: &str = "feature not implemented";  

pub fn feature_not_implemented<T>() -> Result<T>
{

    Err(Error::msg(FEATURE_NOT_IMPLEMETED))

}

static STORED_VALUE_IS_ZERO: &str = "Stored value is zero";

pub fn stored_value_is_zero<T>() -> Result<T>
{

    Err(Error::msg(STORED_VALUE_IS_ZERO))

}

static PROVIDED_VALUE_IS_ZERO: &str = "Provided value is zero";

pub fn provided_value_is_zero<T>() -> Result<T>
{

    Err(Error::msg(PROVIDED_VALUE_IS_ZERO))

}


