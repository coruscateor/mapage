use crate::types::unit_value::UnitValue;

use paste::paste;

use async_graphql::{Result, Error, ErrorExtensions};

use crate::errors::*;

use crate::types::async_graphql_values::{I128Scalar, U128Scalar};

use crate::print_thread_id_feature;

//https://docs.rs/scc/0.11.1/scc/hash_map/struct.HashMap.html

//namespace was scc_hashmap

//insert_async

#[macro_export]
macro_rules! impl_store_namespace_insert
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _insert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String,
            {

                crate::print_thread_id_feature!([<$label _insert>]);

                self.$field.insert(key, value).await

            }

        }

    }

}

//call methods...

//update_async

//returns unit and not the value_type because of vector types

#[macro_export]
macro_rules! impl_store_namespace_update
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String,
            {

                crate::print_thread_id_feature!([<$label _update>]);

                self.$field.update(&key, value).await

            }

        }

    }

}

//key_by_ref


#[macro_export]
macro_rules! impl_store_namespace_try_replace
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_replace>](&self, key: $key_type, value: $value_type) -> Option<$value_type> //key: String,
            {

                crate::print_thread_id_feature!([<$label _try_replace>]);

                self.$field.try_replace(&key, value).await

            }

        }

    }

}

//update_fn

#[macro_export]
macro_rules! impl_store_namespace_update_fn
{

    ($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _update_fn>]);

                self.$field.update_fn(key, $updater_fn_name).await

            }

        }

    }

}

//update_fn - params

#[macro_export]
macro_rules! impl_store_namespace_update_fn_param
{

    ($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $type_param:ty) =>
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type, param: $type_param) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _update_fn_fn_1_param>]);
        
                self.$field.update_fn_1_param(key, $updater_fn_name, param).await
        
            }

        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_update_fn_params
{

    ($param_count:stmt, $label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $($parameter_name:ident: $type_param:ty),*) =>
    {

        paste! {

            pub async fn [<$label _update_fn>](&self, key: $key_type, $($parameter_name:ident: $type_param:ty),*) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _update_fn_params>]);

                self.$field.[<update_fn_params $param_count>](key, $updater_fn_name, $($parameter_name),*).await

            }

        }

    }

}

//upsert_async - copy

#[macro_export]
macro_rules! impl_store_namespace_upsert_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str>
            {

                crate::print_thread_id_feature!([<$label  _upsert_copy>]);

                self.$field.upsert_copy(key, value).await

            }

        }

    }

}

//upsert_async - clone

#[macro_export]
macro_rules! impl_store_namespace_upsert_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: $key_type, value: $value_type) -> async_graphql::Result<&'static str> //<UnitValue> //key: String, 
            {

                crate::print_thread_id_feature!([<$label  _upsert_clone>]);

                self.$field.upsert_clone(key, value).await

            }

        }

    }

}

//remove_async

#[macro_export]
macro_rules! impl_store_namespace_remove
{

    ($label:ident, $field:ident, $key_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _remove>](&self, key: $key_type) -> async_graphql::Result<&'static str> //<UnitValue>
            {

                crate::print_thread_id_feature!([<$label  _remove>]);

                self.$field.remove(&key).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_try_retrieve
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _try_retrieve>](&self, key: $key_type) -> Option<$value_type>
            {

                crate::print_thread_id_feature!([<$label _try_retrieve>]);

                self.$field.try_retrieve(&key).await

            }

        }

    }

}


//remove_if_async

//read_async - copy

#[macro_export]
macro_rules! impl_store_namespace_read_copy
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _read_copy>]);

                self.$field.read_copy(&key).await

            }

        }

    }

}

//key_by_ref

#[macro_export]
macro_rules! impl_store_namespace_key_by_ref_read_copy
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _key_by_ref_read>](&self, key: &$key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _key_by_ref_read_copy>]);

                self.$field.read_copy(key).await

            }

        }

    }

}


//read_async - clone

#[macro_export]
macro_rules! impl_store_namespace_read_clone
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _read_clone>]);

                self.$field.read_clone(&key).await

            }

        }

    }

}

//key_by_ref

#[macro_export]
macro_rules! impl_store_namespace_key_by_ref_read_clone
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _key_by_ref_read>](&self, key: &$key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _key_by_ref_read_clone>]);

                self.$field.read_clone(key).await

            }

        }

    }

}

//read_async - try read - copy

#[macro_export]
macro_rules! impl_store_namespace_try_read_copy
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_read>](&self, key: $key_type) -> Option<$return_type>
            {

                crate::print_thread_id_feature!([<$label _try_read_copy>]);

                self.$field.try_read_copy(&key).await

            }

        }

    }

}

//read_async - try read - clone

#[macro_export]
macro_rules! impl_store_namespace_try_read_clone
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _try_read>](&self, key: $key_type) -> Option<$return_type>
            {

                crate::print_thread_id_feature!([<$label _try_read_clone>]);

                self.$field.try_read_clone(&key).await

            }

        }

    }

}

//functions

//read_fn

#[macro_export]
macro_rules! impl_store_namespace_read_fn
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        paste! {

            pub async fn [<$label _read_fn](&self, key: $key_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _read_fn>]);

                //let updater = $get_reader_fn();

                self.$field.read_fn(&key, $reader_fn_name);

            }

        }

    }

}

//read_fn - params

#[macro_export]
macro_rules! impl_store_namespace_read_fn_param
{

    ($label:ident, $field:ident, $key_type:ty, $param_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        //single

        paste! {

            pub async fn [<$label _read_fn_param>](&self, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>
            {

                crate::print_thread_id_feature!([<$label _read_fn_param>]);

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

                crate::print_thread_id_feature!([<$label _read_fn_param>]);

                let param = ($($parameter_name,)*);

                //let reader = $get_reader_fn();

                self.$field.read_fn_param(&key, $reader_fn_name, &param);

            }

        }

    }

}

//contains_async

#[macro_export]
macro_rules! impl_store_namespace_contains
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _contains>](&self, key: $key_type) -> bool
            {

                crate::print_thread_id_feature!([<$label _contains>]);

                self.$field.contains(&key).await

            }

        }

    }

}

//scan_async

//for_each_async

//retain_async

#[macro_export]
macro_rules! impl_store_namespace_clear
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _clear>](&self) -> &'static str
            {

                crate::print_thread_id_feature!([<$label _clear>]);

                self.$field.clear().await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_clear_and_get_len
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _clear_and_get_len>](&self) -> usize
            {

                crate::print_thread_id_feature!([<$label _clear_and_get_len>]);

                self.$field.clear_and_get_len().await

            }

        }

    }

}

//len

#[macro_export]
macro_rules! impl_store_namespace_len
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _len>](&self) -> usize
            {

                crate::print_thread_id_feature!([<$label _len>]);

                self.$field.len().await

            }

        }

    }

}

//is_empty

#[macro_export]
macro_rules! impl_store_namespace_is_empty
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _is_empty>](&self) -> bool
            {

                crate::print_thread_id_feature!([<$label _is_empty>]);

                self.$field.is_empty().await

            }

        }

    }

}

//capacity

#[macro_export]
macro_rules! impl_store_namespace_capacity
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _capacity>](&self) -> usize
            {

                crate::print_thread_id_feature!([<$label _capacity>]);

                self.$field.capacity().await

            }

        }

    }

}

//get_all_keys

#[macro_export]
macro_rules! impl_store_namespace_get_all_keys_copy
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _get_all_keys>](&self) -> HashSet<$key_type>
            {

                crate::print_thread_id_feature!([<$label _get_all_keys_copy>]);

                self.$field.get_all_keys_copy().await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_get_all_keys_clone
{

    ($label:ident, $field:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _get_all_keys>](&self) -> HashSet<$key_type>
            {

                crate::print_thread_id_feature!([<$label _get_all_keys_clone>]);

                self.$field.get_all_keys_clone().await

            }

        }

    }

}

//Methods

//Copy

#[macro_export]
macro_rules! impl_store_namespace_all_methods_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_insert!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_update!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_try_replace!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_upsert_copy!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_remove!($label, $field, $key_type);

        crate::impl_store_namespace_try_retrieve!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_read_copy!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_try_read_copy!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_contains!($label, $field, $key_type);

        crate::impl_store_namespace_clear!($label, $field);

        crate::impl_store_namespace_clear_and_get_len!($label, $field);

        crate::impl_store_namespace_len!($label, $field);

        crate::impl_store_namespace_is_empty!($label, $field);

        crate::impl_store_namespace_capacity!($label, $field);

    }

}

//Clone

#[macro_export]
macro_rules! impl_store_namespace_all_methods_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_insert!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_update!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_try_replace!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_upsert_clone!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_remove!($label, $field, $key_type);

        crate::impl_store_namespace_try_retrieve!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_read_clone!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_try_read_clone!($label, $field, $key_type, $value_type);

        crate::impl_store_namespace_contains!($label, $field, $key_type);

        crate::impl_store_namespace_clear!($label, $field);

        crate::impl_store_namespace_clear_and_get_len!($label, $field);

        crate::impl_store_namespace_len!($label, $field);

        crate::impl_store_namespace_is_empty!($label, $field);

        crate::impl_store_namespace_capacity!($label, $field);

    }

}

//keys, values

#[macro_export]
macro_rules! impl_store_namespace_all_methods_key_copy_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_get_all_keys_copy!($label, $field, $key_type);

        crate::impl_store_namespace_all_methods_value_copy!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_store_namespace_all_methods_key_copy_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_get_all_keys_copy!($label, $field, $key_type);

        crate::impl_store_namespace_all_methods_value_clone!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_store_namespace_all_methods_key_clone_value_copy
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_get_all_keys_clone!($label, $field, $key_type);

        crate::impl_store_namespace_all_methods_value_copy!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_store_namespace_all_methods_key_clone_value_clone
{

    ($label:ident, $field:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_namespace_get_all_keys_clone!($label, $field, $key_type);

        crate::impl_store_namespace_all_methods_value_clone!($label, $field, $key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_store_namespace_floating_point_ops
{

    ($label:ident, $key_type:ty, $return_type:ty, $type_param:ty, $inc_fn:ident, $dec_fn:ident) => //$field:ident, 
    {

        paste! {

            //add

            crate::impl_store_namespace_update_fn_param!([<$label _add>], [<$label _values>], $key_type, add, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _add_self>], [<$label _values>], $key_type, add_self, $return_type);

            //div

            crate::impl_store_namespace_update_fn_param!([<$label _div>], [<$label _values>], $key_type, div, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _div_self>], [<$label _values>], $key_type, div_self, $return_type);

            //mul

            crate::impl_store_namespace_update_fn_param!([<$label _mul>], [<$label _values>], $key_type, mul, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _mul_self>], [<$label _values>], $key_type, mul_self, $return_type);

            //neg

            crate::impl_store_namespace_update_fn!([<$label _neg>], [<$label _values>], $key_type, neg, $return_type);

            //rem

            crate::impl_store_namespace_update_fn_param!([<$label _rem>], [<$label _values>], $key_type, rem, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _rem_self>], [<$label _values>], $key_type, rem_self, $return_type);

            //sub

            crate::impl_store_namespace_update_fn_param!([<$label _sub>], [<$label _values>], $key_type, sub, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _sub_self>], [<$label _values>], $key_type, sub_self, $return_type);

            //non-trait

            //inc

            crate::impl_store_namespace_update_fn!([<$label _inc>], [<$label _values>], $key_type, $inc_fn, $return_type);

            //dec

            crate::impl_store_namespace_update_fn!([<$label _dec>], [<$label _values>], $key_type, $dec_fn, $return_type);

            //[< inc ::< $type_param , $has_one > >]

        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_signed_integer_ops
{

    ($label:ident, $key_type:ty, $return_type:ty, $type_param:ty, $inc_fn:ident, $dec_fn:ident) => //$field:ident, 
    {

        paste! {

            //add

            crate::impl_store_namespace_update_fn_param!([<$label _add>], [<$label _values>], $key_type, add, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _add_self>], [<$label _values>], $key_type, add_self, $return_type);

            //bit_and

            crate::impl_store_namespace_update_fn_param!([<$label _bit_and>], [<$label _values>], $key_type, bit_and, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_and_self>], [<$label _values>], $key_type, bit_and_self, $return_type);

            //bit_or

            crate::impl_store_namespace_update_fn_param!([<$label _bit_or>], [<$label _values>], $key_type, bit_or, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_or_self>], [<$label _values>], $key_type, bit_or_self, $return_type);

            //bit_xor

            crate::impl_store_namespace_update_fn_param!([<$label _bit_xor>], [<$label _values>], $key_type, bit_xor, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_xor_self>], [<$label _values>], $key_type, bit_xor_self, $return_type);

            //div

            crate::impl_store_namespace_update_fn_param!([<$label _div>], [<$label _values>], $key_type, div, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _div_self>], [<$label _values>], $key_type, div_self, $return_type);

            //mul

            crate::impl_store_namespace_update_fn_param!([<$label _mul>], [<$label _values>], $key_type, mul, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _mul_self>], [<$label _values>], $key_type, mul_self, $return_type);

            //neg

            crate::impl_store_namespace_update_fn!([<$label _neg>], [<$label _values>], $key_type, neg, $return_type);

            //not

            crate::impl_store_namespace_update_fn!([<$label _not>], [<$label _values>], $key_type, not, $return_type);

            //rem

            crate::impl_store_namespace_update_fn_param!([<$label _rem>], [<$label _values>], $key_type, rem, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _rem_self>], [<$label _values>], $key_type, rem_self, $return_type);

            //shl

            crate::impl_store_namespace_update_fn_param!([<$label _shl>], [<$label _values>], $key_type, shl, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _shl_self>], [<$label _values>], $key_type, shl_self, $return_type);

            //shr

            crate::impl_store_namespace_update_fn_param!([<$label _shr>], [<$label _values>], $key_type, shr, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _shr_self>], [<$label _values>], $key_type, shr_self, $return_type);

            //sub

            crate::impl_store_namespace_update_fn_param!([<$label _sub>], [<$label _values>], $key_type, sub, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _sub_self>], [<$label _values>], $key_type, sub_self, $return_type);

            //inc

            crate::impl_store_namespace_update_fn!([<$label _inc>], [<$label _values>], $key_type, $inc_fn, $return_type);

            //dec

            crate::impl_store_namespace_update_fn!([<$label _dec>], [<$label _values>], $key_type, $dec_fn, $return_type);


        }

    }

}

#[macro_export]
macro_rules! impl_store_namespace_unsigned_integer_ops
{

    ($label:ident, $key_type:ty, $return_type:ty, $type_param:ty, $inc_fn:ident, $dec_fn:ident) => //$field:ident, 
    {

        paste! {

            //add

            crate::impl_store_namespace_update_fn_param!([<$label _add>], [<$label _values>], $key_type, add, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _add_self>], [<$label _values>], $key_type, add_self, $return_type);

            //bit_and

            crate::impl_store_namespace_update_fn_param!([<$label _bit_and>], [<$label _values>], $key_type, bit_and, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_and_self>], [<$label _values>], $key_type, bit_and_self, $return_type);

            //bit_or

            crate::impl_store_namespace_update_fn_param!([<$label _bit_or>], [<$label _values>], $key_type, bit_or, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_or_self>], [<$label _values>], $key_type, bit_or_self, $return_type);

            //bit_xor

            crate::impl_store_namespace_update_fn_param!([<$label _bit_xor>], [<$label _values>], $key_type, bit_xor, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _bit_xor_self>], [<$label _values>], $key_type, bit_xor_self, $return_type);

            //div

            crate::impl_store_namespace_update_fn_param!([<$label _div>], [<$label _values>], $key_type, div, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _div_self>], [<$label _values>], $key_type, div_self, $return_type);

            //mul

            crate::impl_store_namespace_update_fn_param!([<$label _mul>], [<$label _values>], $key_type, mul, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _mul_self>], [<$label _values>], $key_type, mul_self, $return_type);

            //No neg

            //not

            crate::impl_store_namespace_update_fn!([<$label _not>], [<$label _values>], $key_type, not, $return_type);

            //rem

            crate::impl_store_namespace_update_fn_param!([<$label _rem>], [<$label _values>], $key_type, rem, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _rem_self>], [<$label _values>], $key_type, rem_self, $return_type);

            //shl

            crate::impl_store_namespace_update_fn_param!([<$label _shl>], [<$label _values>], $key_type, shl, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _shl_self>], [<$label _values>], $key_type, shl_self, $return_type);

            //shr

            crate::impl_store_namespace_update_fn_param!([<$label _shr>], [<$label _values>], $key_type, shr, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _shr_self>], [<$label _values>], $key_type, shr_self, $return_type);

            //sub

            crate::impl_store_namespace_update_fn_param!([<$label _sub>], [<$label _values>], $key_type, sub, $return_type, $type_param);

            crate::impl_store_namespace_update_fn!([<$label _sub_self>], [<$label _values>], $key_type, sub_self, $return_type);

            //inc

            crate::impl_store_namespace_update_fn!([<$label _inc>], [<$label _values>], $key_type, $inc_fn, $return_type);

            //dec

            crate::impl_store_namespace_update_fn!([<$label _dec>], [<$label _values>], $key_type, $dec_fn, $return_type);


        }

    }

}
