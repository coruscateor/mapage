use std::collections::HashSet;

use delegate::delegate;

type KeyType = String;

use crate::types::async_graphql_values::SelectedType;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, SelectedType>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, SelectedType>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct SelectedTypeNamespace
{

    namespace: Namespace

}

impl SelectedTypeNamespace
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: Namespace::new()

        }

    }

    //KeyType keys

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: KeyType, value: SelectedType) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: SelectedType) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: SelectedType) -> Option<SelectedType>;

            pub async fn update_fn<R, FN: FnOnce(&mut SelectedType) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut SelectedType) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &KeyType) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<SelectedType>;

            pub async fn read_fn<R, FN: FnOnce(&SelectedType) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &SelectedType) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: SelectedType) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_clone(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> async_graphql::Result<SelectedType>
    {

        self.namespace.read_clone(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<SelectedType>
    {

        self.namespace.try_read_clone(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.get_all_keys_clone().await

    }

}

