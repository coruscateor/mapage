use async_graphql::{Object, Context};

use crate::types::sizes::size_of_char;

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_only_move_key};

type KeyType = crate::types::keys::CharKeyType;

#[derive(Default)]
pub struct CharQuery;

#[cfg(any(feature = "all_types", feature = "char"))]
#[Object]
impl CharQuery
{

    async fn size_of_char(&self) -> usize
    {
        
        size_of_char()

    }

    //

    pub async fn char_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<char>
    {

        call_store_method!(ctx, get_char_namespace_ref, read, key)

    }

    pub async fn char_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<char>
    {

        call_store_method!(ctx, get_char_namespace_ref, try_read, key)

    }

    pub async fn char_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_char_namespace_ref, contains, key)

    }

    pub async fn char_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, len)

    }

    pub async fn char_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, is_empty)

    }

    pub async fn char_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, capacity)

    }

    pub async fn char_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, get_all_keys)

    }
    

}

#[cfg(not(any(feature = "all_types", feature = "char")))]
#[Object]
impl CharQuery 
{

    #[graphql(visible = false)]
    pub async fn char_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct CharMutation;

#[cfg(any(feature = "all_types", feature = "char"))]
#[Object]
impl CharMutation
{

    pub async fn char_insert(&self, ctx: &Context<'_>, key: KeyType, value: char) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_char_namespace_ref, insert, key, value)

    }

    pub async fn char_update(&self, ctx: &Context<'_>, key: KeyType, value: char) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_char_namespace_ref, update, key, value)

    }

    pub async fn char_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: char) -> Option<char>
    {

        call_store_method!(ctx, get_char_namespace_ref, try_replace, key, value)

    }

    pub async fn char_upsert(&self, ctx: &Context<'_>, key: KeyType, value: char) -> async_graphql::Result<&'static str>
    {

        call_store_method_only_move_key!(ctx, get_char_namespace_ref, upsert, key, value)

    }

    pub async fn char_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_char_namespace_ref, remove, key)

    }

    pub async fn char_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<char>
    {

        call_store_method!(ctx, get_char_namespace_ref, try_retrieve, key)

    }

    pub async fn char_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, clear)

    }

    pub async fn char_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_char_namespace_ref, clear_and_get_len)

    }

}

#[cfg(not(any(feature = "all_types", feature = "char")))]
#[Object]
impl CharMutation
{

    #[graphql(visible = false)]
    pub async fn char_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

