use crate::types::unit_value::UnitValue;

#[macro_export]
macro_rules! impl_unwarp_mutex_field
{

    ($field_name:ident, $method_name:ident) =>
    {

        pub fn $method_name(&self) -> UnitValue
        {

            {

                let mut mg = self.$field_name.lock().unwrap();

                mg.$method_name();

            }

            UnitValue::new()

        }

    };
    ($field_name:ident, $method_name:ident, $returns:ty) =>
    {

        pub fn $method_name(&self) -> $returns
        {

            let mut mg = self.$field_name.lock().unwrap();

            mg.$method_name()

        }

    };
    ($field_name:ident, $method_name:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        //paste! {

            pub fn $method_name(&self, $($parameter_name: $name_type,)*) -> UnitValue //[<$method_name>]
            {

                {

                    let mut mg = self.$field_name.lock().unwrap();

                    mg.$method_name($($parameter_name: $name_type,)*);

                }

                UnitValue::new()

            }

        //}

    };
    ($field_name:ident, $method_name:ident, $returns:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        //paste! {

            pub fn $method_name(&self, $($parameter_name: $name_type,)*) -> $returns //[<$method_name>]
            {

                let mut mg = self.$field_name.lock().unwrap();

                mg.$method_name($($parameter_name: $name_type,)*)

            }

        //}

    }

}

#[macro_export]
macro_rules! unwarp_lock_result
{

    ($name:ident) =>
    {


        $name.lock().unwrap()

    }

}
