use std::collections::HashSet;

use delegate::delegate;

type KeyType = String;

use crate::types::async_graphql_values::Whatever;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, Whatever>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, Whatever>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct WhateverNamespace
{

    namespace: Namespace

}

impl WhateverNamespace
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: Namespace::new()

        }

    }

    //Whatever keys

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: KeyType, value: Whatever) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: Whatever) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: Whatever) -> Option<Whatever>;

            pub async fn update_fn<R, FN: FnOnce(&mut Whatever) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;
            
            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut Whatever) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &KeyType) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<Whatever>;

            pub async fn read_fn<R, FN: FnOnce(&Whatever) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &Whatever) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: Whatever) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_clone(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> async_graphql::Result<Whatever>
    {

        self.namespace.read_clone(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<Whatever>
    {

        self.namespace.try_read_clone(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.get_all_keys_clone().await

    }

}

