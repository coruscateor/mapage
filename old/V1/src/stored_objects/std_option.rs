use super::stored_unit::*;

use crate::identifier::*;

use crate::{consts::*, all_types};

//use crate::async_graphql_value_containers::

use crate::
{
    
    stored_object::StoredObject, 
    //value::*,
    all_types::*,
    errors::invalid_operation_instance_is_immuatable,
    uniary_operations::*, async_graphql_value_containers::*, binary_operations::Bop,
    errors::*,
    //logic::Bool

};

use std::cell::Cell;

//#[derive(Default)]
pub struct StdOption<T>
    where T: StoredObject
{

    option: Option<T>,
    my_type: Type

}

impl<T> StdOption<T>
    where T: StoredObject
{
    
    pub fn new(my_type: Type) -> Self
    {

        Self
        {

            option: None,
            my_type

        }

    }

    pub fn init(stored_object: T) -> Self //, all_types: &AllTypes
    {

        let my_type = stored_object.get_type(); //(all_types);

        Self
        {

            option: Some(stored_object),
            my_type

        }

    }

}

/*
impl StoredUnit
{

    pub fn get_value(&self) -> &str
    {

        get_unit_str()

    }

}
*/

impl<T> StoredObject for StdOption<T>
    where T: StoredObject
{

    fn get_type(&self) -> Type //, all_types: &AllTypes
    {

        /*
        Type::Option
        {
            
            param: all_types.get_rcd_type(&self.my_type)
        
        }
        */

        AllTypes::get_option_type(self.my_type.clone())

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>> //, identifier: KeyNS
    {

        if let Some(val) = self.option.as_ref()
        {

            return val.get_value()

        }

        Ok(None)
        
    }

    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        if let Some(val) = self.option.as_ref()
        {

            return val.get_bool();

        }

        stored_option_is_none_error()
        
    }

}