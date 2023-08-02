use super::stored_unit::*;

use crate::async_graphql_value_containers::{AnyObject, CharValue};
use crate::identifier::*;

use crate::
{
    
    stored_object::StoredObject, 
    //value::*,
    all_types::*,
    errors::invalid_operation_instance_is_immuatable,
    uniary_operations::*, async_graphql_value_containers::{NumericOrBool, NumericOrIdentifierOrBool, Numeric}, binary_operations::Bop,
    errors::*,
    identifier::*

};

#[derive(Default)]
pub struct StoredIdentifier
{

    value: Identifier

}

impl StoredIdentifier
{
    
    pub fn new(value: Identifier) -> Self
    {

        Self
        {

            value

        }

    }

}

impl StoredObject for StoredIdentifier
{

    fn get_type(&self) -> Type
    {

        AllTypes::get_char_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {
        
        Ok(Some(AnyObject::Identifier(self.value.clone())))

    }

}
