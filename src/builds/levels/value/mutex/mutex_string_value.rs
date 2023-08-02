use crate::types::{string_value::StringValue, unit_value::UnitValue};

use crate::impl_unwarp_mutex_field;

use corlib::has_one::*;

use std::ops::*;



type mtx<T> = std::sync::Mutex<T>;

#[derive(Default)]
pub struct MutexStringValue
{

    value: mtx<StringValue>

}

impl MutexStringValue
{

    pub fn new(value: String) -> Self
    {

        Self
        {

            value: mtx::new(StringValue::new(value))

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            value: mtx::new(StringValue::with_capacity(capacity))

        }

    }

    impl_unwarp_mutex_field!(value, get_value, String);

    impl_unwarp_mutex_field!(value, set_value, value: String);

}


impl From<String> for MutexStringValue
{
    
    fn from(value: String) -> Self
    {
        
        MutexStringValue::new(value) 
    
    }
    
}

