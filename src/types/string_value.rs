use super::UnitValue;

use paste::paste;

use corlib::{impl_get};

use crate::impl_set;

use std::convert::From;

#[derive(Default)]
pub struct StringValue
{

    value: String

}

//https://doc.rust-lang.org/std/string/struct.String.html

impl StringValue
{
    
    pub fn new(value: String) -> Self
    {

        Self
        {

            value

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            value: String::with_capacity(capacity)

        }

    }

    impl_get!(value, String);

    impl_set!(value, String);

}

impl From<String> for StringValue
{
    fn from(value: String) -> Self
    {
        
        StringValue::new(value) 
    
    }
    
}

