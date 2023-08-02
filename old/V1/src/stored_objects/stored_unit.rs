use crate::async_graphql_value_containers::AnyObject;
use crate::consts::*;

//use super::stored_unit::*;

use crate::identifier::*;

use crate::
{
    
    stored_object::StoredObject, 
    //value::*,
    all_types::*,
    errors::invalid_operation_instance_is_immuatable,
    uniary_operations::*, async_graphql_value_containers::{NumericOrBool, NumericOrIdentifierOrBool, Numeric}, binary_operations::Bop,
    errors::*,
    //logic::Bool

};

use std::cell::Cell;

#[derive(Default)]
pub struct StoredUnit();

impl StoredUnit
{
    
    pub fn new() -> Self
    {

        Self()

    }
    
    pub fn get_value(&self) -> &str
    {

        get_unit_str()

    }

}

impl StoredObject for StoredUnit
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::unit

        AllTypes::get_unit_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Unit(UnitValue::new()))) //::new(self.value)))))

    }
    
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        invalid_operation()
        
    }

}