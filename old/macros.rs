use crate::types::UnitValue;

use paste::paste;

#[macro_export]
macro_rules! impl_set
{

    ($name:ident, $name_type:ty) =>
    {

        //pub trait_impl_get_ref!($name, $name_type)

        paste!
        {

            pub fn [<set_ $name>](&mut self, value: $name_type) -> UnitValue
            {

                self.$name = value;

                UnitValue::new()

            }

        }

    }

}