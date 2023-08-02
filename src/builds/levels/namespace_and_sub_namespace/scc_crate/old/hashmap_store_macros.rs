use crate::types::unit_value::UnitValue;

use paste::paste;

use async_graphql::{Result, Error, ErrorExtensions};

use crate::errors::*;

use crate::types::async_graphql_values::{I128Scalar, U128Scalar};

//https://docs.rs/scc/0.11.1/scc/hash_map/struct.HashMap.html

//insert_async

#[macro_export]
macro_rules! impl_scc_hashmap_insert
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _insert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String,
            {

                self.$field.insert(key, value).await

            }

        }

    }

}

//call methods...

//update_async

//returns unit and not the value_type because of vector types

#[macro_export]
macro_rules! impl_scc_hashmap_update
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String,
            {

                self.$field.update(&key, value).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_try_replace
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_replace>](&self, key: $key_type, value: $value_type) -> Option<$value_type> //key: String,
            {

                self.$field.try_replace(&key, value).await

            }

        }

    }

}

//update_fn

#[macro_export]
macro_rules! impl_scc_hashmap_update_fn
{

    ($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type) -> async_graphql::Result<$return_type> //, updater: F //_fn //<UnitValue>
                //where F: FnOnce(&K, &mut V) -> R,
            {

                //let updater = $get_updater_fn();

                self.$field.update_fn(key, $updater_fn_name).await

            }

        }

    }

}

//update_fn - params

#[macro_export]
macro_rules! impl_common_update_fn_param
{

    ($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $type_param:ty) => //ident //$parameter_name:ident, 
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type, param: $type_param) -> async_graphql::Result<$return_type> //<$type_param, R>
            {
        
                self.$field.update_fn_1_param(key, $updater_fn_name, param).await //$parameter_name
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_common_update_fn_params
{

    ($param_count:stmt, $label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $($parameter_name:ident: $type_param:ty),*) =>
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type, $($parameter_name:ident: $type_param:ty),*) -> async_graphql::Result<$return_type>
            {

                self.$field.[<update_fn_params $param_count>](key, $updater_fn_name, $($parameter_name),*).await

            }

        }

    }

}

/*
#[macro_export]
macro_rules! impl_scc_hashmap_update_fn_param
{

    ($label:ident, $field:ident, $key_type:ty, $param_type:ty, $updater_fn_name:ident, $return_type:ty) =>
    {

        //single

        paste! {

            pub async fn [<$label _update_fn_param>](&self, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type> //<K, P, R> //, updater: F //_fn //<UnitValue>
                //where F: FnOnce(&K, &mut V) -> R,
            {

                //let updater = $get_updater_fn();

                self.$field.update_fn_param(key, $updater_fn_name, param).await

            }

        }

    };
    ($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) => //$param_type:ty, 
    {

        //multi

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type> //$param_type, //, updater: F //_fn //<UnitValue>
            {

                let param = ($($parameter_name,)*);

                //let updater = $get_updater_fn();

                self.$field.update_fn_param(key, $updater_fn_name, param).await

            }

        }

    }

}
*/

//update_fns_convert - Convert resulting value

/*
#[macro_export]
macro_rules! impl_scc_hashmap_update_fns_convert
{

    ($label:ident, $field:ident, key: $key_type:ty, $value_type:ty, $get_updater_fn:ident, $get_converter_fn:ident, $return_type:ty) =>
    {

        //no params

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type) -> async_graphql::Result<$return_type:> //, updater: F //_fn //<UnitValue>
                //where F: FnOnce(&K, &mut V) -> R,
            {

                let updater = $get_updater_fn();

                let converter = $get_converter_fn();

                self.$field.update_fn_convert(&key, updater, converter);

            }

        }

    };
}

#[macro_export]
macro_rules! impl_scc_hashmap_update_fns_convert_param
{

    ($label:ident, $field:ident, key: $key_type:ty, $param_type:ty, $get_updater_fn:ident, $get_converter_fn:ident, $return_type:ty) =>
    {

        //param

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type> //, updater: F //_fn //<UnitValue>
                //where F: FnOnce(&K, &mut V) -> R,
            {

                let updater = $get_updater_fn();

                let converter = $get_converter_fn();

                self.$field.update_fns_convert_param(&key, updater, converter, param);

            }

        }

    };
    ($label:ident, $field:ident, key: $key_type:ty, $get_updater_fn:ident, $get_converter_fn:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        //multiple params

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type:ty> //, updater: F //_fn //<UnitValue>
            {

                let param = ($($parameter_name,)*);

                let updater = $get_updater_fn();

                let converter = $get_converter_fn();

                self.$field.update_fn_convert_param(&key, updater, param);

            }

        }

    }

}
*/

//

//upsert_async

/*
#[macro_export]
macro_rules! impl_scc_hashmap_upsert
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue> //key: String, 
            {

                self.$field.upsert(key, value).await

            }

        }

    }

}
*/

//upsert_async - copy

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String, 
            {

                self.$field.upsert_copy(key, value).await

            }

        }

    }

}

//upsert_async - clone

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String, 
            {

                self.$field.upsert_clone(key, value).await

            }

        }

    }

}

//remove_async

#[macro_export]
macro_rules! impl_scc_hashmap_remove
{

    ($label:ident, $field:ident, $key_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _remove>](&self, key: $key_type) -> async_graphql::Result<&'static str> //<UnitValue>
            {

                self.$field.remove(&key).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_try_retrieve
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _try_retrieve>](&self, key: $key_type) -> Option<$value_type>
            {

                self.$field.try_retrieve(&key).await

            }

        }

    }

}


//remove_if_async

//read_async - copy

#[macro_export]
macro_rules! impl_scc_hashmap_read_copy
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                 self.$field.read_copy(&key).await

            }

        }

    }

}

//read_async - clone

#[macro_export]
macro_rules! impl_scc_hashmap_read_clone
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                self.$field.read_clone(&key).await

            }

        }

    }

}

//read_async - try read - copy

#[macro_export]
macro_rules! impl_scc_hashmap_try_read_copy
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_read>](&self, key: $key_type) -> Option<$return_type>
            {

                self.$field.try_read_copy(&key).await

            }

        }

    }

}

//read_async - try read - clone

#[macro_export]
macro_rules! impl_scc_hashmap_try_read_clone
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_read>](&self, key: $key_type) -> Option<$return_type>
            {

                self.$field.try_read_clone(&key).await

            }

        }

    }

}

//functions

//read_fn

#[macro_export]
macro_rules! impl_scc_hashmap_read_fn
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        paste! {

            pub async fn [<$label _read_fn](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                //let updater = $get_reader_fn();

                self.$field.read_fn(&key, $reader_fn_name);

            }

        }

    }

}

//read_fn - params

#[macro_export]
macro_rules! impl_scc_hashmap_read_fn_param
{

    ($label:ident, $field:ident, $key_type:ty, $param_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        //single

        paste! {

            pub async fn [<$label _read_fn_param>](&self, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>
            {

                //let reader = $get_reader_fn();

                self.$field.read_fn_param(&key, $reader_fn_name, &param);

            }

        }

    };
    ($label:ident, $field:ident, $key_type:ty, $return_type:ty, $reader_fn_name:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        //multi

        paste! {

            pub async fn [<$label _read_fn_param>](&self, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let param = ($($parameter_name,)*);

                //let reader = $get_reader_fn();

                self.$field.read_fn_param(&key, $reader_fn_name, &param);

            }

        }

    }

}

//read_fns_convert - Convert resulting value

/*
#[macro_export]
macro_rules! impl_scc_hashmap_read_fns_convert
{

    ($label:ident, $field:ident, key: $key_type:ty, $get_reader_fn:ident, $get_converter_fn:ident, $return_type:ty) =>
    {

        //no params

        paste! {

            pub async fn [<$label _read>]<F, P, IR, C, R>(&self, key: K) -> async_graphql::Result<$return_type>
            {

                let updater = $get_reader_fn();

                let converter = $get_converter_fn();

                self.$field.read_fn_convert(&key, reader, converter);

            }

        }

    };
}

#[macro_export]
macro_rules! impl_scc_hashmap_read_fns_convert_param
{

    ($label:ident, $field:ident, key: $key_type:ty, $value_type:ty, $get_reader_fn:ident, $get_converter_fn:ident, $return_type:ty) =>
    {

        //param

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type, param: $value_type) -> async_graphql::Result<$return_type>
            {

                let reader = $get_reader_fn();

                let converter = $get_converter_fn();

                self.$field.read_fns_convert_param(&key, reader, converter, &param);

            }

        }

    };
    ($label:ident, $field:ident, key: $key_type:ty, get_reader_fn:ident, get_converter_fn:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        //multiple params

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let param = ($($parameter_name,)*);

                let updater = $get_reader_fn();

                let converter = $get_converter_fn();

                self.$field.read_fn_convert_param(&key, reader, &param);

            }

        }

    }

}
*/

//

//contains_async

#[macro_export]
macro_rules! impl_scc_hashmap_contains
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _contains>](&self, key: $key_type) -> bool
            {

                self.$field.contains(&key).await

            }

        }

    }

}

//scan_async

//for_each_async

//retain_async

//clear_async

#[macro_export]
macro_rules! impl_scc_hashmap_clear
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _clear>](&self)
            {

                self.$field.clear().await;

            }

        }

    }

}

#[macro_export]
macro_rules! impl_common_hashmap_clear_and_get_len
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _clear_and_get_len>](&self) -> usize
            {

                self.$field.clear_and_get_len().await

            }

        }

    }

}

//len

#[macro_export]
macro_rules! impl_scc_hashmap_len
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _len>](&self) -> usize
            {

                self.$field.len().await

            }

        }

    }

}

//is_empty

#[macro_export]
macro_rules! impl_scc_hashmap_is_empty
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _is_empty>](&self) -> bool
            {

                self.$field.is_empty().await

            }

        }

    }

}

//capacity

#[macro_export]
macro_rules! impl_scc_hashmap_capacity
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _capacity>](&self) -> usize
            {

                self.$field.capacity().await

            }

        }

    }

}

//get_all_keys

#[macro_export]
macro_rules! impl_scc_hashmap_get_all_keys_copy
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _get_all_keys>](&self) -> HashSet<$key_type> //Vec<$key_type>
            {

                self.$field.get_all_keys_copy().await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_get_all_keys_clone
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _get_all_keys>](&self) -> HashSet<$key_type> //Vec<$key_type>
            {

                self.$field.get_all_keys_clone().await

            }

        }

    }

}

//Essential methods

//Copy

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_insert!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_update!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_try_replace!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_upsert_copy!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_remove!($label, $field, $key_type);

        crate::impl_scc_hashmap_try_retrieve!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_read_copy!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_try_read_copy!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_contains!($label, $field, $key_type);

        crate::impl_scc_hashmap_clear!($label, $field);

        crate::impl_scc_hashmap_len!($label, $field);

        crate::impl_scc_hashmap_is_empty!($label, $field);

        crate::impl_scc_hashmap_capacity!($label, $field);

    }

}

//Clone

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_insert!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_update!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_try_replace!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_upsert_clone!($label, $field, $key_type, $value_type);

        //crate::impl_scc_hashmap_upsert_copy!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_remove!($label, $field, $key_type);

        crate::impl_scc_hashmap_try_retrieve!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_read_clone!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_try_read_clone!($label, $field, $key_type, $value_type);

        crate::impl_scc_hashmap_contains!($label, $field, $key_type);

        crate::impl_scc_hashmap_clear!($label, $field);

        crate::impl_scc_hashmap_len!($label, $field);

        crate::impl_scc_hashmap_is_empty!($label, $field);

        crate::impl_scc_hashmap_capacity!($label, $field);

    }

}

//keys, values

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_key_copy_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_get_all_keys_copy!($label, $field, $key_type);

        crate::impl_scc_hashmap_all_methods_value_copy!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_key_copy_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_get_all_keys_copy!($label, $field, $key_type);

        crate::impl_scc_hashmap_all_methods_value_clone!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_key_clone_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_get_all_keys_clone!($label, $field, $key_type);

        crate::impl_scc_hashmap_all_methods_value_copy!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_key_clone_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_get_all_keys_clone!($label, $field, $key_type);

        crate::impl_scc_hashmap_all_methods_value_clone!($label, $field, $key_type, $value_type);

    }

}

//--

//get all

//call methods

//scalar type method calls

//update_async

/*
#[macro_export]
macro_rules! impl_scc_hashmap_update_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { v.$method(); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { v.$method() });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.update_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.update_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//upsert_async

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method(); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method() });

                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method($($parameter_name: $name_type,)*) });

                Ok(UnitValue::new())

            }

        }

    };

}

//

#[macro_export]
macro_rules! impl_scc_hashmap_update_upsert_method_calls
{

    ($label:ident, $field:ident, $method:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $return_type:ty) =>
    {
        
        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type, $return_type);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type,$return_type);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type, $($parameter_name:ident: $name_type:ty)*);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type, $($parameter_name:ident: $name_type:ty)*);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type,$return_type, $($parameter_name:ident: $name_type:ty)*);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type, $return_type, $($parameter_name:ident: $name_type:ty)*);

    };

}

//read_async

#[macro_export]
macro_rules! impl_scc_hashmap_read_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { v.$method(); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { v.$method() });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.read_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//scalar type function calls

//update_async

#[macro_export]
macro_rules! impl_scc_hashmap_update_function_call
{

    ($label:ident, $field:ident, $function:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { $function(v); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { $function(v) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.update_async(key, |_, v| { $function(v, $($parameter_name,)*); /*()*/ }); //: $name_type

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.update_async(key, |_, v| { $function(v, $($parameter_name,)*) }); //: $name_type

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//upsert_async

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_function_call
{

    ($label:ident, $field:ident, $function:ident, $value_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v) });

                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v, $($parameter_name,)*); /*()*/ }).await; //: $name_type
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v, $($parameter_name,)*) }).await; //: $name_type

                Ok(UnitValue::new())

            }

        }

    };

}

//

#[macro_export]
macro_rules! impl_scc_hashmap_update_upsert_function_calls
{

    ($label:ident, $field:ident, $function:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty) =>
    {
        
        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $return_type);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $return_type);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $($parameter_name: $name_type)*);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $($parameter_name: $name_type)*);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $return_type, $($parameter_name: $name_type)*);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $return_type, $($parameter_name: $name_type)*);

    };

}

//read_async

#[macro_export]
macro_rules! impl_scc_hashmap_read_function_call
{

    ($label:ident, $field:ident, $function:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $function>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { $function(v); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { $function(v) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.read_async(key, |_, v| { $function(v, $($parameter_name)*); /*()*/ }); //: $name_type,

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| { $function(v, $($parameter_name,)*) }); //: $name_type

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}
*/

//extract result if method returns result



//vector type method calls


