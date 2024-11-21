//use crate::{impl_store_container_capacity, impl_store_container_clear, impl_get_type_sizes, impl_store_container_contains, impl_store_container_get_all_keys, impl_store_container_insert, impl_store_container_is_empty, impl_store_container_len, impl_store_container_read, impl_store_container_remove, impl_store_container_update, impl_store_container_upsert}; //, impl_store_container_get_all_keys};

#[macro_export]
macro_rules! impl_scc_hashmap_model_mutation_methods
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_container_insert!($label, $key_type, $value_type);

        crate::impl_store_container_update!($label, $key_type, $value_type);

        crate::impl_store_container_upsert!($label, $key_type, $value_type);

        crate::impl_store_container_remove!($label, $key_type);

        crate::impl_store_container_clear!($label);
        
    }

}

#[macro_export]
macro_rules! impl_scc_hashmap_model_query_methods
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {

        crate::impl_store_container_read!($label, $key_type, $value_type);

        crate::impl_store_container_contains!($label, $key_type);

        crate::impl_store_container_len!($label);

        crate::impl_store_container_is_empty!($label);

        crate::impl_store_container_capacity!($label);

        crate::impl_store_container_get_all_keys!($label, $key_type);

    }

}

/*
#[macro_export]
macro_rules! impl_scc_hashmap_model_bool_query_fn_methods
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {


    }

}
*/

#[macro_export]
macro_rules! impl_scc_hashmap_model_bool_mutation_fn_methods
{

    //($label:ident, $key_type:ty, $value_type:ty)
    ($key_type:ty) =>
    {

        crate::impl_store_container_update_fn_param!(bool_bit_and, $key_type, bool, bool);

        crate::impl_store_container_update_fn_param!(bool_bit_or, $key_type, bool, bool);
    
        crate::impl_store_container_update_fn_param!(bool_bit_xor, $key_type, bool, bool);
    
        crate::impl_store_container_update_fn!(bool_bit_and_self, $key_type, bool);
    
        crate::impl_store_container_update_fn!(bool_bit_or_self, $key_type, bool);
    
        crate::impl_store_container_update_fn!(bool_bit_xor_self, $key_type, bool);

    }

}

