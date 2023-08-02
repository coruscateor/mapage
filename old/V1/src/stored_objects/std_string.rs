use super::stored_unit::*;

use crate::identifier::*;

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

#[derive(Default)]
pub struct StdString
{

    value: String
    //Rcd/cached string?

}

impl StdString
{
    
    pub fn new(value: String) -> Self
    {

        Self
        {

            value

        }

    }

}

impl StoredObject for StdString
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::String

        AllTypes::get_string_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>> //, identifier: KeyNS
    {

        //self.value.get_value(identifier)
        
        Ok(Some(AnyObject::String(StringValue::new(self.value.clone()))))

    }

    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        invalid_operation()
        
    }

}

//Methods

