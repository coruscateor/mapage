
use super::UnitValue;

use paste::paste;

use crate::
{
    
    //stored_object::StoredObject, 
    //value::*,
    //all_types::*,
    //errors::invalid_operation_instance_is_immuatable,
    //uniary_operations::*, async_graphql_value_containers::{NumericOrBool, NumericOrIdentifierOrBool, Numeric, BoolValue}, binary_operations::Bop,
    //errors::*,
    //logic::Bool

};

use corlib::{impl_get}; //, impl_set};

//use crate::macros::impl_set;

use crate::impl_set;

#[derive(Default)]
pub struct BoolValue
{

    //my_type: Type
    value: bool

}

impl BoolValue
{
    
    pub fn new(value: bool) -> Self
    {

        Self
        {

            value

        }

    }

    impl_get!(value, bool);

    impl_set!(value, bool);

    pub fn not(&mut self) -> bool
    {

        self.value = !self.value;

        self.value

    }

    pub fn bit_and(&mut self, right_side: bool) -> bool
    {

        self.value = self.value & right_side;

        self.value

    }

    pub fn bit_or(&mut self, right_side: bool) -> bool
    {

        self.value = self.value | right_side;

        self.value

    }

    pub fn bit_xor(&mut self, right_side: bool) -> bool
    {

        self.value = self.value ^ right_side;

        self.value

    }

    pub fn bit_and_self(&mut self) -> bool
    {

        self.value = self.value & self.value;

        self.value

    }

    pub fn bit_or_self(&mut self) -> bool
    {

        self.value = self.value | self.value;

        self.value

    }

    pub fn bit_xor_self(&mut self) -> bool
    {

        self.value = self.value ^ self.value;

        self.value

    }

}

impl From<bool> for BoolValue
{
    fn from(value: bool) -> Self
    {
        
        BoolValue::new(value) 
    
    }
    
}

