use async_graphql::{Object, Context};

#[cfg(any(feature = "all_types", feature = "Whatever"))]
use crate::{types::{sizes::size_of_whatever, async_graphql_values::{Whatever, InputOneOfWhatever}}};

use super::StoreType;

use std::collections::HashSet;

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

#[cfg(any(feature = "all_types", feature = "Whatever"))]
type KeyType = crate::types::keys::WhateverKeyType;

#[derive(Default)]
pub struct WhateverQuery;

#[cfg(any(feature = "all_types", feature = "Whatever"))]
#[Object]
impl WhateverQuery 
{

    async fn size_of_whatever(&self) -> usize
    {

        size_of_whatever()

    }

    //

    pub async fn whatever_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Whatever>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, read, key)

    }

    pub async fn whatever_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<Whatever>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, try_read, key)

    }

    pub async fn whatever_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_whatever_namespace_ref, contains, key)

    }

    pub async fn whatever_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, len)

    }

    pub async fn whatever_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, is_empty)

    }

    pub async fn whatever_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, capacity)

    }

    pub async fn whatever_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, get_all_keys)

    }

}

#[cfg(not(any(feature = "all_types", feature = "Whatever")))]
#[Object]
impl WhateverQuery 
{

    #[graphql(visible = false)]
    pub async fn whatever_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct WhateverMutation;

#[cfg(any(feature = "all_types", feature = "Whatever"))]
#[Object]
impl WhateverMutation
{

    pub async fn whatever_insert(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfWhatever) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_whatever_namespace_ref, insert, key, value.into())

    }

    pub async fn whatever_update(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfWhatever) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, update, key, value.into())

    }

    pub async fn whatever_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfWhatever) -> Option<Whatever>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, try_replace, key, value.into())

    }

    pub async fn whatever_upsert(&self, ctx: &Context<'_>, key: KeyType, value: InputOneOfWhatever) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_whatever_namespace_ref, upsert, key, value.into())

    }

    pub async fn whatever_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, remove, key)

    }

    pub async fn whatever_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<Whatever>
    {

        call_store_method!(ctx, get_whatever_namespace_ref, try_retrieve, key)

    }

    pub async fn whatever_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, clear)

    }

    pub async fn whatever_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_whatever_namespace_ref, clear_and_get_len)

    }

    //methods...

}

#[cfg(not(any(feature = "all_types", feature = "Whatever")))]
#[Object]
impl WhateverMutation
{

    #[graphql(visible = false)]
    pub async fn whatever_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

