use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_u128, async_graphql_values::U128Scalar}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

#[cfg(any(feature = "all_types", feature = "u128"))]
type KeyType = crate::types::keys::U128KeyType;

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

    pub async fn u128_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, read, key)

    }

    pub async fn u128_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, try_read, key)

    }

    pub async fn u128_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
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

    pub async fn u128_insert(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_u128_namespace_ref, insert, key, value)

    }

    pub async fn u128_update(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u128_namespace_ref, update, key, value)

    }

    pub async fn u128_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> Option<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, try_replace, key, value)

    }

    pub async fn u128_upsert(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_u128_namespace_ref, upsert, key, value)

    }

    pub async fn u128_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_u128_namespace_ref, remove, key)

    }

    pub async fn u128_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<U128Scalar>
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

    //ops

    //add
    
    pub async fn u128_add_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, add_op, key, value)

    }

    pub async fn u128_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, add_self_op, key)

    }

    //bit_and

    pub async fn u128_bit_and_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_and_op, key, value)

    }

    pub async fn u128_bit_and_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_and_self_op, key)

    }

    //bit_or

    pub async fn u128_bit_or_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_or_op, key, value)

    }

    pub async fn u128_bit_or_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_or_self_op, key)

    }

    //bit_xor

    pub async fn u128_bit_xor_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_xor_op, key, value)

    }

    pub async fn u128_bit_xor_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, bit_xor_self_op, key)

    }

    //div

    pub async fn u128_div_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, div_op, key, value)

    }

    pub async fn u128_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn u128_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, mul_op, key, value)

    }

    pub async fn u128_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, mul_self_op, key)

    }

    //no neg op

    //not

    pub async fn u128_not_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, not_op, key)

    }

    //rem

    pub async fn u128_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, rem_op, key, value)

    }

    pub async fn u128_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, rem_self_op, key)

    }

    //shl

    pub async fn u128_shl_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, shl_op, key, value)

    }

    pub async fn u128_shl_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, shl_self_op, key)

    }

    //shr

    pub async fn u128_shr_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, shr_op, key, value)

    }

    pub async fn u128_shr_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, shr_self_op, key)

    }

    //sub

    pub async fn u128_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: U128Scalar) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, sub_op, key, value)

    }

    pub async fn u128_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn u128_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn u128_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<U128Scalar>
    {

        call_store_method!(ctx, get_u128_namespace_ref, dec_op, key)

    }

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
