use std::collections::HashSet;

use delegate::delegate;

use anyhow::Result;

type KeyType = crate::types::keys::CharKeyType;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, char>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, char>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct CharNamespace
{

    namespace: Namespace

}

impl CharNamespace
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

            pub async fn insert(&self, key: KeyType, value: char) -> Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: char) -> Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: char) -> Option<char>;

            pub async fn update_fn<R, FN: FnOnce(&mut char) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut char) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn remove(&self, key: &KeyType) -> Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<char>;

            pub async fn read_fn<R, FN: FnOnce(&char) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &char) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: char) -> Result<&'static str>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> Result<char>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<char>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.get_all_keys_clone().await

    }

}

