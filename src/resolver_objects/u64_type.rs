use async_graphql::{Object, Context};

use crate::{types::sizes::size_of_u64}; //, UnitValue}}; //, builds::levels::sub_namespace::StoreType::StoreType};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

type KeyType = crate::types::keys::U64KeyType;

#[derive(Default)]
pub struct U64Query;

#[cfg(any(feature = "all_types", feature = "u64"))]
#[Object]
impl U64Query
{

    async fn size_of_u64(&self) -> usize
    {

        size_of_u64()

    }

    //

    pub async fn u64_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, read, key)

    }

    pub async fn u64_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, try_read, key)

    }

    pub async fn u64_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_u64_namespace_ref, contains, key)

    }

    pub async fn u64_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, len)

    }

    pub async fn u64_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, is_empty)

    }

    pub async fn u64_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, capacity)

    }

    pub async fn u64_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<String>
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "u64")))]
#[Object]
impl U64Query 
{

    #[graphql(visible = false)]
    pub async fn u64_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct U64Mutation;

#[cfg(any(feature = "all_types", feature = "u64"))]
#[Object]
impl U64Mutation
{

    pub async fn u64_insert(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_u64_namespace_ref, insert, key, value)

    }

    pub async fn u64_update(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u64_namespace_ref, update, key, value)

    }

    pub async fn u64_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> Option<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, try_replace, key, value)

    }

    pub async fn u64_upsert(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_u64_namespace_ref, upsert, key, value)

    }

    pub async fn u64_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u64_namespace_ref, remove, key)

    }

    pub async fn u64_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, try_retrieve, key)

    }

    pub async fn u64_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, clear)

    }

    pub async fn u64_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_u64_namespace_ref, clear_and_get_len)

    }

    //ops

    //add

    pub async fn u64_add_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, add_op, key, value)

    }

    pub async fn u64_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, add_self_op, key)

    }

    //bit_and

    pub async fn u64_bit_and_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_and_op, key, value)

    }

    pub async fn u64_bit_and_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_and_self_op, key)

    }

    //bit_or

    pub async fn u64_bit_bit_or_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_or_op, key, value)

    }

    pub async fn u64_bit_bit_or_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_or_self_op, key)

    }

    //bit_xor

    pub async fn u64_bit_bit_xor_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_xor_op, key, value)

    }

    pub async fn u64_bit_bit_xor_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, bit_xor_self_op, key)

    }

    //div

    pub async fn u64_div_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, div_op, key, value)

    }

    pub async fn u64_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn u64_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, mul_op, key, value)

    }

    pub async fn u64_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, mul_self_op, key)

    }

    //no neg

    //not

    pub async fn u64_not_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, not_op, key)

    }

    //rem

    pub async fn u64_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, rem_op, key, value)

    }

    pub async fn u64_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, rem_self_op, key)

    }

    //shl

    pub async fn u64_shl_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, shl_op, key, value)

    }

    pub async fn u64_shl_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, shl_self_op, key)

    }

    //shr

    pub async fn u64_shr_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, shr_op, key, value)

    }

    pub async fn u64_shr_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, shr_self_op, key)

    }

    //sub

    pub async fn u64_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: u64) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, sub_op, key, value)

    }

    pub async fn u64_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn u64_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn u64_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<u64>
    {

        call_store_method!(ctx, get_u64_namespace_ref, dec_op, key)

    }

}

#[cfg(not(any(feature = "all_types", feature = "u64")))]
#[Object]
impl U64Mutation
{

    #[graphql(visible = false)]
    pub async fn u64_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
