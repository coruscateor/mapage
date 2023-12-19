use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_i8}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

#[cfg(any(feature = "all_types", feature = "i8"))]
type KeyType = crate::types::keys::I8KeyType;

#[derive(Default)]
pub struct I8Query;

#[cfg(any(feature = "all_types", feature = "i8"))]
#[Object]
impl I8Query 
{

    async fn size_of_i8(&self) -> usize
    {

        size_of_i8()

    }

    //

    pub async fn i8_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, read, key)

    }

    pub async fn i8_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, try_read, key)

    }

    pub async fn i8_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_i8_namespace_ref, contains, key)

    }

    pub async fn i8_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, len)

    }

    pub async fn i8_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, is_empty)

    }

    pub async fn i8_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, capacity)

    }
    
    pub async fn i8_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "i8")))]
#[Object]
impl I8Query 
{

    #[graphql(visible = false)]
    pub async fn i8_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct I8Mutation;

#[cfg(any(feature = "all_types", feature = "i8"))]
#[Object]
impl I8Mutation
{
    
    pub async fn i8_insert(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_i8_namespace_ref, insert, key, value)

    }

    pub async fn i8_update(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_i8_namespace_ref, update, key, value)

    }

    pub async fn i8_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> Option<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, try_replace, key, value)

    }

    pub async fn i8_upsert(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_i8_namespace_ref, upsert, key, value)

    }

    pub async fn i8_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_i8_namespace_ref, remove, key)

    }

    pub async fn i8_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, try_retrieve, key)

    }
    
    pub async fn i8_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, clear)

    }

    pub async fn i8_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_i8_namespace_ref, clear_and_get_len)

    }

    //ops

    //add

    pub async fn i8_add_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, add_op, key, value)

    }

    pub async fn i8_add_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, add_self_op, key)

    }

    //bit_and

    pub async fn i8_bit_and_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_and_op, key, value)

    }

    pub async fn i8_bit_and_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_and_self_op, key)

    }

    //bit_or

    pub async fn i8_bit_or_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_or_op, key, value)

    }

    pub async fn i8_bit_or_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_or_self_op, key)

    }

    //bit_xor

    pub async fn i8_bit_xor_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_xor_op, key, value)

    }

    pub async fn i8_bit_xor_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, bit_xor_self_op, key)

    }

    //div

    pub async fn i8_div_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, div_op, key, value)

    }

    pub async fn i8_div_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, div_self_op, key)

    }

    //mul

    pub async fn i8_mul_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, mul_op, key, value)

    }

    pub async fn i8_mul_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, mul_self_op, key)

    }

    //neg

    pub async fn i8_neg_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, neg_op, key)

    }

    //not

    pub async fn i8_not_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, not_op, key)

    }

    //rem

    pub async fn i8_rem_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, rem_op, key, value)

    }

    pub async fn i8_rem_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, rem_self_op, key)

    }

    //shl

    pub async fn i8_shl_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, shl_op, key, value)

    }

    pub async fn i8_shl_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, shl_self_op, key)

    }

    //shr

    pub async fn i8_shr_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, shr_op, key, value)

    }

    pub async fn i8_shr_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, shr_self_op, key)

    }

    //sub

    pub async fn i8_sub_op(&self, ctx: &Context<'_>, key: KeyType, value: i8) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, sub_op, key, value)

    }

    pub async fn i8_sub_self_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, sub_self_op, key)

    }

    //inc

    pub async fn i8_inc_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, inc_op, key)

    }

    //dec

    pub async fn i8_dec_op(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<i8>
    {

        call_store_method!(ctx, get_i8_namespace_ref, dec_op, key)

    }

}

#[cfg(not(any(feature = "all_types", feature = "i8")))]
#[Object]
impl I8Mutation
{

    #[graphql(visible = false)]
    pub async fn i8_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
