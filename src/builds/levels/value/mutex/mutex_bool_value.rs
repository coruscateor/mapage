use crate::types::{bool_value::BoolValue, unit_value::UnitValue};

//use super::macros::*;

use crate::impl_unwarp_mutex_field;

type Mtx<T> = std::sync::Mutex<T>;

#[derive(Default)]
pub struct MutexBoolValue
{

    value: Mtx<BoolValue>

}

impl MutexBoolValue
{

    //pub fn new(value: CharValue) -> Self
    pub fn new(value: bool) -> Self
    {

        Self
        {

            value: Mtx::new(BoolValue::new(value)) //(BoolValue::default())
            
        }

    }

    /*
    pub fn init(value: bool) -> Self
    {

        Self
        {

                value: mtx::new(BoolValue::new(value))
            
        }

    }
    */

    impl_unwarp_mutex_field!(value, get_value, bool);

    impl_unwarp_mutex_field!(value, set_value, value: bool);

    impl_unwarp_mutex_field!(value, not, bool);

    impl_unwarp_mutex_field!(value, bit_and, value: bool);

    impl_unwarp_mutex_field!(value, bit_or, value: bool);

    impl_unwarp_mutex_field!(value, bit_xor, value: bool);

    impl_unwarp_mutex_field!(value, bit_and_self, bool);

    impl_unwarp_mutex_field!(value, bit_or_self, bool);

    impl_unwarp_mutex_field!(value, bit_xor_self, bool);

}

