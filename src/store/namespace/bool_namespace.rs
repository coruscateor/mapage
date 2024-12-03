use std::collections::HashSet;

use delegate::delegate;

use crate::{types::ops::*, impl_update_fn_op_method};

use paste::paste;

use anyhow::Result;

type KeyType = crate::types::keys::BoolKeyType;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace = SCC_HashMapNamespace<KeyType, bool>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace = DashMapNamespace<KeyType, bool>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct BoolNamespace
{

    namespace: Namespace

}

impl BoolNamespace
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

            pub async fn insert(&self, key: KeyType, value: bool) -> Result<()>;

            pub async fn update(&self, key: &KeyType, value: bool) -> Result<()>;

            pub async fn replace(&self, key: &KeyType, value: bool) -> Result<()>;

            pub async fn try_replace(&self, key: &KeyType, value: bool) -> Option<bool>;

            pub async fn update_fn<R, FN: FnOnce(&mut bool) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&KeyType, &mut bool) -> Result<R>>(&self, key: &KeyType, updater: FN) -> Result<R>;

            pub async fn remove(&self, key: &KeyType) -> Result<()>;

            pub async fn retrieve(&self, key: &KeyType) -> Result<bool>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<bool>;

            pub async fn read_fn<R, FN: FnOnce(&bool) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&KeyType, &bool) -> Result<R>>(&self, key: &KeyType, reader: FN) -> Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self);

            pub async fn len_then_clear(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: bool) -> Result<()>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> Result<bool>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<bool>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.all_keys_clone().await

    }

    impl_update_fn_op_method!(not, KeyType, bool);

    impl_update_fn_op_method!(bit_and, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_and_self, KeyType, bool);

    impl_update_fn_op_method!(bit_or, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_or_self, KeyType, bool);

    impl_update_fn_op_method!(bit_xor, KeyType, bool, value: bool);

    impl_update_fn_op_method!(bit_xor_self, KeyType, bool);

}

