use async_graphql::{Object, Context};

use crate::{types::{sizes::{size_of_unit_value, size_of_unit_value_combined}, UnitValue}}; //, builds::levels::sub_namespace::StoreType::StoreType};

use super::StoreType;

use std::collections::HashSet;

#[derive(Default)]
pub struct UnitValueQuery;

#[Object]
impl UnitValueQuery
{

    async fn size_of_unit_value(&self) -> usize
    {

        size_of_unit_value()

    }

    async fn size_of_unit_value_combined(&self) -> usize
    {

        size_of_unit_value_combined()

    }

    pub async fn unit_value_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<StoreType>().unit_value_read(key).await

    }

    pub async fn unit_value_try_read(&self, ctx: &Context<'_>, key: String) -> Option<UnitValue>
    {

        ctx.data_unchecked::<StoreType>().unit_value_try_read(key).await

    }

    pub async fn unit_value_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<StoreType>().unit_value_contains(key).await

    }

    pub async fn unit_value_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<StoreType>().unit_value_len().await

    }

    pub async fn unit_value_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<StoreType>().unit_value_is_empty().await

    }

    pub async fn unit_value_capacity(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<StoreType>().unit_value_capacity().await

    }

    pub async fn unit_value_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<String> //Vec<String>
    {

        ctx.data_unchecked::<StoreType>().unit_value_get_all_keys().await

    }

}

#[derive(Default)]
pub struct UnitValueMutation;

#[Object]
impl UnitValueMutation
{

    pub async fn unit_value_insert(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<StoreType>().unit_value_insert(key, UnitValue::new()).await

    }

    pub async fn unit_value_update(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<StoreType>().unit_value_update(key, UnitValue::new()).await

    }

    pub async fn unit_value_try_replace(&self, ctx: &Context<'_>, key: String) -> Option<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<StoreType>().unit_value_try_replace(key, UnitValue::new()).await

    }

    pub async fn unit_value_upsert(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<StoreType>().unit_value_upsert(key, UnitValue::new()).await

    }

    pub async fn unit_value_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<StoreType>().unit_value_remove(key).await

    }

    pub async fn unit_value_try_retrieve(&self, ctx: &Context<'_>, key: String) -> Option<UnitValue>
    {

        ctx.data_unchecked::<StoreType>().unit_value_try_retrieve(key).await

    }

    pub async fn unit_value_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<StoreType>().unit_value_clear().await

    }

}