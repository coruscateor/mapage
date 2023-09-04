use std::collections::HashSet;

use delegate::delegate;

use crate::{types::ops::*, impl_update_fn_op_method};

use paste::paste;

type KeyType = crate::types::keys::VecBoolKeyType;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, Vec<bool>>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, Vec<bool>>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct BoolVecNamespace
{

    namespace: Namespace

}

impl BoolVecNamespace
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

            pub async fn insert(&self, key: KeyType, value: Vec<bool>) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: Vec<bool>) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: Vec<bool>) -> Option<Vec<bool>>;

            pub async fn update_fn<R, FN: FnMut(&mut Vec<bool>) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnMut(&KeyType, &mut Vec<bool>) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &KeyType) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<Vec<bool>>;

            pub async fn read_fn<R, FN: Fn(&Vec<bool>) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: Fn(&KeyType, &Vec<bool>) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: KeyType, value: Vec<bool>) -> async_graphql::Result<&'static str>;

            pub async fn read(&self, key: &KeyType) -> async_graphql::Result<Vec<bool>>;

            pub async fn try_read(&self, key: &KeyType) -> Option<Vec<bool>>;

            pub async fn get_all_keys(&self) -> HashSet<KeyType>;

        }
    }

    /*
    impl_update_fn_op_method!(not, KeyType, bool);

    impl_update_fn_op_method!(bit_and, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_and_self, KeyType, bool);

    impl_update_fn_op_method!(bit_or, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_or_self, KeyType, bool);

    impl_update_fn_op_method!(bit_xor, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_xor_self, KeyType, bool);
    */

}
