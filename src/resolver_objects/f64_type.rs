use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_f64}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

type KeyType = crate::types::keys::F64KeyType;

#[derive(Default)]
pub struct F64Query;

#[cfg(any(feature = "all_types", feature = "f64"))]
#[Object]
impl F64Query
{

    async fn size_of_f64(&self) -> usize
    {

        size_of_f64()

    }

    //

    pub async fn f64_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, read, key)

    }

    pub async fn f64_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, try_read, key)

    }

    pub async fn f64_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_f64_namespace_ref, contains, key)

    }

    pub async fn f64_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, len)

    }

    pub async fn f64_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, is_empty)

    }

    pub async fn f64_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, capacity)

    }

    pub async fn f64_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "f64")))]
#[Object]
impl F64Query 
{

    #[graphql(visible = false)]
    pub async fn f64_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct F64Mutation;

#[cfg(any(feature = "all_types", feature = "f64"))]
#[Object]
impl F64Mutation
{

    pub async fn f64_insert(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_f64_namespace_ref, insert, key, value)

    }

    pub async fn f64_update(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_f64_namespace_ref, update, key, value)

    }

    pub async fn f64_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> Option<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, try_replace, key, value)

    }

    pub async fn f64_upsert(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_f64_namespace_ref, upsert, key, value)

    }

    pub async fn f64_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_f64_namespace_ref, remove, key)

    }

    pub async fn f64_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, try_retrieve, key)

    }

    pub async fn f64_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, clear)

    }

    pub async fn f64_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f64_namespace_ref, clear_and_get_len)

    }

    //ops

    //add

    pub async fn f64_add_op(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, add_op, key, value)

    }

    pub async fn f64_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, add_self_op, key)

    }

    //div

    pub async fn f64_div_op(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, div_op, key, value)

    }

    pub async fn f64_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn f64_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, mul_op, key, value)

    }

    pub async fn f64_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, mul_self_op, key)

    }

    //neg

    pub async fn f64_neg_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, neg_op, key)

    }

    //rem

    pub async fn f64_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, rem_op, key, value)

    }

    pub async fn f64_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, rem_self_op, key)

    }

    //sub

    pub async fn f64_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: f64) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, sub_op, key, value)

    }

    pub async fn f64_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn f64_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn f64_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f64>
    {

        call_store_method!(ctx, get_f64_namespace_ref, dec_op, key)

    }

}

#[cfg(not(any(feature = "all_types", feature = "f64")))]
#[Object]
impl F64Mutation
{

    #[graphql(visible = false)]
    pub async fn f64_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
