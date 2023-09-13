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

    pub async fn i128_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
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

    //ops

    //add
    
    pub async fn i128_add_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, add_op, key, value)

    }

    pub async fn i128_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, add_self_op, key)

    }

    //bit_and

    pub async fn i128_bit_and_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_and_op, key, value)

    }

    pub async fn i128_bit_and_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_and_self_op, key)

    }

    //bit_or

    pub async fn i128_bit_or_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_or_op, key, value)

    }

    pub async fn i128_bit_or_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_or_self_op, key)

    }

    //bit_xor

    pub async fn i128_bit_xor_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_xor_op, key, value)

    }

    pub async fn i128_bit_xor_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, bit_xor_self_op, key)

    }

    //div

    pub async fn i128_div_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, div_op, key, value)

    }

    pub async fn i128_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn i128_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, mul_op, key, value)

    }

    pub async fn i128_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, mul_self_op, key)

    }

    //neg

    pub async fn i128_neg_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, neg_op, key)

    }

    //not

    pub async fn i128_not_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, not_op, key)

    }

    //rem

    pub async fn i128_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, rem_op, key, value)

    }

    pub async fn i128_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, rem_self_op, key)

    }

    //shl

    pub async fn i128_shl_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, shl_op, key, value)

    }

    pub async fn i128_shl_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, shl_self_op, key)

    }

    //shr

    pub async fn i128_shr_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, shr_op, key, value)

    }

    pub async fn i128_shr_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, shr_self_op, key)

    }

    //sub

    pub async fn i128_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, sub_op, key, value)

    }

    pub async fn i128_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn i128_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn i128_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_i128_namespace_ref, dec_op, key)

    }
    
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

