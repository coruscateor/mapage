use async_graphql::{Object, Context};

use crate::{types::{sizes::size_of_string}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

type KeyType = crate::types::keys::StringKeyType;

#[derive(Default)]
pub struct StringQuery;

#[cfg(any(feature = "all_types", feature = "String"))]
#[Object]
impl StringQuery 
{

    async fn size_of_string(&self) -> usize
    {

        size_of_string()

    }

    //

    pub async fn string_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<String>
    {

        call_store_method!(ctx, get_string_namespace_ref, read, key)

    }

    pub async fn string_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<String>
    {

        call_store_method!(ctx, get_string_namespace_ref, try_read, key)

    }

    pub async fn string_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_string_namespace_ref, contains, key)

    }

    pub async fn string_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, len)

    }

    pub async fn string_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, is_empty)

    }

    pub async fn string_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, capacity)

    }

    pub async fn string_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "String")))]
#[Object]
impl StringQuery 
{

    #[graphql(visible = false)]
    pub async fn string_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}


#[derive(Default)]
pub struct StringMutation;

#[cfg(any(feature = "all_types", feature = "String"))]
#[Object]
impl StringMutation
{

    pub async fn string_insert(&self, ctx: &Context<'_>, key: KeyType, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_string_namespace_ref, insert, key, value)

    }

    pub async fn string_update(&self, ctx: &Context<'_>, key: KeyType, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_string_namespace_ref, update, key, value)

    }

    pub async fn string_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: String) -> Option<String>
    {

        call_store_method!(ctx, get_string_namespace_ref, try_replace, key, value)

    }

    pub async fn string_upsert(&self, ctx: &Context<'_>, key: KeyType, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_string_namespace_ref, upsert, key, value)

    }

    pub async fn string_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_string_namespace_ref, remove, key)

    }

    pub async fn string_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<String>
    {

        call_store_method!(ctx, get_string_namespace_ref, try_retrieve, key)

    }

    pub async fn string_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, clear)

    }

    pub async fn string_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_string_namespace_ref, clear_and_get_len)

    }
    
    //methods...
    
}

#[cfg(not(any(feature = "all_types", feature = "String")))]
#[Object]
impl StringMutation
{

    #[graphql(visible = false)]
    pub async fn string_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
