use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_u128, async_graphql_values::U128Scalar}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

#[derive(Default)]
pub struct U128Query;

#[cfg(any(feature = "all_types", feature = "u128"))]
#[Object]
impl U128Query 
{

    async fn size_of_u128(&self) -> usize
    {

        size_of_u128()

    }

    //

    pub async fn u128_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, read, key)

    }

    pub async fn u128_try_read(&self, ctx: &Context<'_>, key: String) -> Option<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, try_read, key)

    }

    pub async fn u128_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        call_store_method!(ctx, get_u128_namespace_ref, contains, key)

    }

    pub async fn u128_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, len)

    }

    pub async fn u128_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, is_empty)

    }

    pub async fn u128_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, capacity)

    }

    pub async fn u128_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<String>
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "u128")))]
#[Object]
impl U128Query 
{

    #[graphql(visible = false)]
    pub async fn u128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct U128Mutation;

#[cfg(any(feature = "all_types", feature = "u128"))]
#[Object]
impl U128Mutation
{

    pub async fn u128_insert(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_u128_namespace_ref, insert, key, value)

    }

    pub async fn u128_update(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u128_namespace_ref, update, key, value)

    }

    pub async fn u128_try_replace(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> Option<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, try_replace, key, value)

    }

    pub async fn u128_upsert(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_u128_namespace_ref, upsert, key, value)

    }

    pub async fn u128_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u128_namespace_ref, remove, key)

    }

    pub async fn u128_try_retrieve(&self, ctx: &Context<'_>, key: String) -> Option<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, try_retrieve, key)

    }

    pub async fn u128_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, clear)

    }

    pub async fn u128_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u128_namespace_ref, clear_and_get_len)

    }

    //ops...
    
    

}

#[cfg(not(any(feature = "all_types", feature = "u128")))]
#[Object]
impl U128Mutation
{

    #[graphql(visible = false)]
    pub async fn u128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
