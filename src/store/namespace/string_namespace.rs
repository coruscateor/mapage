use std::collections::HashSet;

use delegate::delegate;

type KeyType = String;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, String>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, String>;

use anyhow::Result;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct StringNamespace
{

    namespace: Namespace

}

impl StringNamespace
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

            pub async fn insert(&self, key: KeyType, value: String) -> Result<()>;

            pub async fn update(&self, key: &KeyType, value: String) -> Result<()>;

            pub async fn replace(&self, key: &KeyType, value: String) -> Result<()>;

            pub async fn try_replace(&self, key: &KeyType, value: String) -> Option<String>;

            pub async fn update_fn<R, FN: FnOnce(&mut String) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;
            
            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut String) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn remove(&self, key: &KeyType) -> Result<()>;

            pub async fn retrieve(&self, key: &KeyType) -> Result<String>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<String>;

            pub async fn read_fn<R, FN: FnOnce(&String) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &String) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> ();

            pub async fn len_then_clear(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: String) -> Result<()>
    {

        self.namespace.upsert_clone(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> Result<String>
    {

        self.namespace.read_clone(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<String>
    {

        self.namespace.try_read_clone(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.all_keys_clone().await

    }

}

