use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_f32}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

type KeyType = crate::types::keys::F32KeyType;

#[derive(Default)]
pub struct F32Query;

#[cfg(any(feature = "all_types", feature = "f32"))]
#[Object]
impl F32Query 
{

    async fn size_of_f32(&self) -> usize
    {

        size_of_f32()

    }

    //

    pub async fn f32_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, read, key)

    }

    pub async fn f32_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, try_read, key)

    }

    pub async fn f32_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_f32_namespace_ref, contains, key)

    }

    pub async fn f32_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, len)

    }

    pub async fn f32_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, is_empty)

    }

    pub async fn f32_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, capacity)

    }

    pub async fn f32_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "f32")))]
#[Object]
impl F32Query 
{

    #[graphql(visible = false)]
    pub async fn f32_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct F32Mutation;

#[cfg(any(feature = "all_types", feature = "f32"))]
#[Object]
impl F32Mutation
{

    pub async fn f32_insert(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_f32_namespace_ref, insert, key, value)

    }

    pub async fn f32_update(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_f32_namespace_ref, update, key, value)

    }

    pub async fn f32_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> Option<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, try_replace, key, value)

    }

    pub async fn f32_upsert(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_f32_namespace_ref, upsert, key, value)

    }

    pub async fn f32_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_f32_namespace_ref, remove, key)

    }

    pub async fn f32_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, try_retrieve, key)

    }

    pub async fn f32_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, clear)

    }

    pub async fn f32_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_f32_namespace_ref, clear_and_get_len)

    }

    //ops

    //add

    pub async fn f32_add_op(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, add_op, key, value)

    }

    pub async fn f32_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, add_self_op, key)

    }

    //div

    pub async fn f32_div_op(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, div_op, key, value)

    }

    pub async fn f32_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn f32_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, mul_op, key, value)

    }

    pub async fn f32_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, mul_self_op, key)

    }

    //neg

    pub async fn f32_neg_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, neg_op, key)

    }

    //rem

    pub async fn f32_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, rem_op, key, value)

    }

    pub async fn f32_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, rem_self_op, key)

    }

    //sub

    pub async fn f32_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: f32) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, sub_op, key, value)

    }

    pub async fn f32_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn f32_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn f32_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<f32>
    {

        call_store_method!(ctx, get_f32_namespace_ref, dec_op, key)

    }

}

#[cfg(not(any(feature = "all_types", feature = "f32")))]
#[Object]
impl F32Mutation
{

    #[graphql(visible = false)]
    pub async fn f32_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
