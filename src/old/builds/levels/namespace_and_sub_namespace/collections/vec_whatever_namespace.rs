use std::collections::HashSet;

use delegate::delegate;

use crate::{types::{ops::*, async_graphql_values::Whatever}, impl_update_fn_op_method};

//use paste::paste;


type KeyType = crate::types::keys::VecWhateverKeyType;

crate::impl_vec_collection_fns_imports!();

#[cfg(feature = "scc_hashmap_namespaces")]
use super::super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, Vec<Whatever>>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, Vec<Whatever>>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct VecWhateverNamespace
{

    namespace: Namespace

}

impl VecWhateverNamespace
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

            pub async fn insert(&self, key: KeyType, value: Vec<Whatever>) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: Vec<Whatever>) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: Vec<Whatever>) -> Option<Vec<Whatever>>;

            pub async fn update_fn<R, FN: FnOnce(&mut Vec<Whatever>) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut Vec<Whatever>) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &KeyType) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<Vec<Whatever>>;

            pub async fn read_fn<R, FN: FnOnce(&Vec<Whatever>) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &Vec<Whatever>) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: KeyType, value: Vec<Whatever>) -> async_graphql::Result<&'static str>;

            pub async fn read(&self, key: &KeyType) -> async_graphql::Result<Vec<Whatever>>;

            pub async fn try_read(&self, key: &KeyType) -> Option<Vec<Whatever>>;

            pub async fn get_all_keys(&self) -> HashSet<KeyType>;

        }
    }

    crate::impl_vec_fns_no_ord_or_partial_eq!(KeyType, Whatever);

}

