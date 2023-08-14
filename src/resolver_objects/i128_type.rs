use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_i128, async_graphql_values::I128Scalar}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

type KeyType = crate::types::keys::I128KeyType;

#[derive(Default)]
pub struct I128Query;

#[cfg(any(feature = "all_types", feature = "i128"))]
#[Object]
impl I128Query
{

    async fn size_of_i128(&self) -> usize
    {

        size_of_i128()

    }

    //

    pub async fn i128_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, read, key)

    }

    pub async fn i128_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, try_read, key)

    }

    pub async fn i128_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_i128_namespace_ref, contains, key)

    }

    pub async fn i128_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, len)

    }

    pub async fn i128_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, is_empty)

    }

    pub async fn i128_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, capacity)

    }

    pub async fn i128_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<String>
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "i128")))]
#[Object]
impl I128Query 
{

    #[graphql(visible = false)]
    pub async fn i128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct I128Mutation;

#[cfg(any(feature = "all_types", feature = "i128"))]
#[Object]
impl I128Mutation
{

    pub async fn i128_insert(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_i128_namespace_ref, insert, key, value)

    }

    pub async fn i128_update(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_i128_namespace_ref, update, key, value)

    }

    pub async fn i128_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> Option<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, try_replace, key, value)

    }

    pub async fn i128_upsert(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_i128_namespace_ref, upsert, key, value)

    }

    pub async fn i128_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_i128_namespace_ref, remove, key)

    }

    pub async fn i128_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, try_retrieve, key)

    }

    pub async fn i128_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, clear)

    }

    pub async fn i128_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i128_namespace_ref, clear_and_get_len)

    }

    //ops...
    
}

#[cfg(not(any(feature = "all_types", feature = "i128")))]
#[Object]
impl I128Mutation
{

    #[graphql(visible = false)]
    pub async fn i128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

