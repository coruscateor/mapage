use crate::types::{char_value::CharValue, unit_value::UnitValue};

//use super::macros::*;

use crate::impl_unwarp_mutex_field;

type Mtx<T> = std::sync::Mutex<T>;

#[derive(Default)]
pub struct MutexCharValue
{

    value: Mtx<CharValue>

}

impl MutexCharValue
{

    //pub fn new(value: CharValue) -> Self
    pub fn new(value: char) -> Self
    {

        Self
        {

            value: Mtx::new(CharValue::new(value)) //(CharValue::default())
            
        }

    }

    /*
    pub fn init(value: char) -> Self
    {

        Self
        {

                value: mtx::new(CharValue::new(value))
            
        }

    }
    */

    impl_unwarp_mutex_field!(value, get_value, char);

    impl_unwarp_mutex_field!(value, set_value, value: char);

}

