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
    //logic::Bool

};

use std::cell::Cell;

#[derive(Default)]
pub struct StoredChar
{

    value: char

}

impl StoredChar
{
    
    pub fn new(value: char) -> Self
    {

        Self
        {

            value

        }

    }

}

impl StoredObject for StoredChar
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::char

        AllTypes::get_char_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        //self.value.get_value(identifier)
        
        Ok(Some(AnyObject::Char(CharValue::new(self.value))))

    }

    /*
    fn is_mutable(&self) -> bool
    {
        
        false

    }
    */

}

/*
impl Default for ImChar
{

    fn default() -> Self
    {

        Self::new(Default::default())

    }

}
*/

/*
#[derive(Default)]
pub struct MutChar
{

    value: Cell<char>

}


impl MutChar
{
    
    pub fn new(value: char) -> Self
    {

        Self
        {

            value: Cell::new(value)

        }

    }

}

impl StoredObject for MutChar
{

    fn get_type(&self, all_types: &AllTypes) -> Type
    {

        Type::char

    }

    /*
    fn uop_mut(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        if let Uop::Not = op
        {

            self.value.set(!self.value.get());

            Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

        }
        else {
            
            invalid_operation()

        }

    }

    fn bop_mut(&self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool>
    {

        let result = get_bool(right_side);

        let res_right_side;

        match result
        {

            Ok(res) =>
            {

                res_right_side = res;

            }
            Err(err) =>
            {

                return Err(err);

            }
            
        }

        let value = self.value.get();

        match op
        {

            Bop::BitAnd =>
            {

                self.value.set(value & res_right_side);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            Bop::BitOr =>
            {

                self.value.set(value | res_right_side);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            Bop::BitXor =>
            {

                self.value.set(value ^ res_right_side);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            _ =>
            {

                invalid_operation()

            }

        }
        
    }

    fn bop_self_mut(&self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {

        let value = self.value.get();
        
        match op
        {

            Bop::BitAnd =>
            {

                self.value.set(value & value);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            Bop::BitOr =>
            {

                self.value.set(value | value);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            Bop::BitXor =>
            {

                self.value.set(value ^ value);

                Ok(NumericOrBool::Bool(Bool::new(self.value.get())))

            }
            _ =>
            {

                invalid_operation()

            }

        }

    }
    */

    fn is_mutable(&self) -> bool
    {
        
        true

    }


}
*/

/*
impl Default for MutChar
{

    fn default() -> Self
    {

        Self::new(Default::default()) 

    }

}
*/