use std::collections::HashSet;

use delegate::delegate;

use crate::{types::ops::*, impl_update_fn_op_method};

use anyhow::Result;

//use paste::paste;


type KeyType = crate::types::keys::VecBoolKeyType;

crate::impl_vec_collection_fns_imports!();

#[cfg(feature = "scc_hashmap_namespaces")]
use super::super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, Vec<bool>>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, Vec<bool>>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct VecBoolNamespace
{

    namespace: Namespace

}

impl VecBoolNamespace
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

            pub async fn insert(&self, key: KeyType, value: Vec<bool>) -> Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: Vec<bool>) -> Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: Vec<bool>) -> Option<Vec<bool>>;

            pub async fn update_fn<R, FN: FnOnce(&mut Vec<bool>) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut Vec<bool>) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn remove(&self, key: &KeyType) -> Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<Vec<bool>>;

            pub async fn read_fn<R, FN: FnOnce(&Vec<bool>) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &Vec<bool>) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: KeyType, value: Vec<bool>) -> Result<&'static str>;

            pub async fn read(&self, key: &KeyType) -> Result<Vec<bool>>;

            pub async fn try_read(&self, key: &KeyType) -> Option<Vec<bool>>;

            pub async fn get_all_keys(&self) -> HashSet<KeyType>;

        }
    }

    crate::impl_vec_fns!(KeyType, bool);

    //Ops

}

