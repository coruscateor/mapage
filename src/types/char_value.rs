use super::UnitValue;

use paste::paste;

use corlib::{impl_get};

use crate::impl_set;

use std::convert::From;

#[derive(Default)]
pub struct CharValue
{

    value: char

}

impl CharValue
{
    
    pub fn new(value: char) -> Self
    {

        Self
        {

            value

        }

    }

    impl_get!(value, char);

    impl_set!(value, char);

}

impl From<char> for CharValue
{
    fn from(value: char) -> Self
    {
        
        CharValue::new(value) 
    
    }
    
}

/*
impl<T> From<T> for CharValue
{
    fn from(_: T) -> Self {
        todo!()
    }
}
*/


