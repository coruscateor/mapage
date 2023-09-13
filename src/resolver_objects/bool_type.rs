use std::collections::HashSet;

use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_bool, OkValue}};

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

use super::StoreType;

type KeyType = crate::types::keys::BoolKeyType;

#[derive(Default)]
pub struct BoolQuery;

#[cfg(any(feature = "all_types", feature = "bool"))]
#[Object]
impl BoolQuery 
{

    async fn size_of_bool(&self) -> usize
    {

        size_of_bool()

    }

    //

    pub async fn bool_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, read, key)

    }

    pub async fn bool_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, try_read, key)

    }

    pub async fn bool_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_bool_namespace_ref, contains, key)

    }

    pub async fn bool_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_bool_namespace_ref, len)

    }

    pub async fn bool_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_bool_namespace_ref, is_empty)

    }

    pub async fn bool_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_bool_namespace_ref, capacity)

    }

    pub async fn bool_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_bool_namespace_ref, get_all_keys)

    }

}


#[cfg(not(any(feature = "all_types", feature = "bool")))]
#[Object]
impl BoolQuery 
{

    #[graphql(visible = false)]
    pub async fn bool_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct BoolMutation;

#[cfg(any(feature = "all_types", feature = "bool"))]
#[Object]
impl BoolMutation
{
    
    pub async fn bool_insert(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_bool_namespace_ref, insert, key, value)

    }

    pub async fn bool_update(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_bool_namespace_ref, update, key, value)

    }

    pub async fn bool_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> Option<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, try_replace, key, value)

    }

    pub async fn bool_upsert(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_bool_namespace_ref, upsert, key, value)

    }

    pub async fn bool_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_bool_namespace_ref, remove, key)

    }

    pub async fn bool_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, try_retrieve, key)

    }

    pub async fn bool_clear(&self, ctx: &Context<'_>) -> &'static str
    {


        call_store_method_no_key!(ctx, get_bool_namespace_ref, clear)

    }

    pub async fn bool_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_bool_namespace_ref, clear_and_get_len)

    }

    //ops

    //with param

    pub async fn bool_not_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {
        
        call_store_method!(ctx, get_bool_namespace_ref, not_op, key)

    }

    pub async fn bool_bit_and_op(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_and_op, key, value)

    }

    pub async fn bool_bit_or_op(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_or_op, key, value)

    }

    pub async fn bool_bit_xor_op(&self, ctx: &Context<'_>, key: KeyType, value: bool) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_xor_op, key, value)

    }

    //on self

    pub async fn bool_bit_and_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_and_self_op, key)

    }

    pub async fn bool_bit_or_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_or_self_op, key)

    }

    pub async fn bool_bit_xor_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_bool_namespace_ref, bit_xor_self_op, key)

    }

}

#[cfg(not(any(feature = "all_types", feature = "bool")))]
#[Object]
impl BoolMutation
{

    #[graphql(visible = false)]
    pub async fn bool_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
