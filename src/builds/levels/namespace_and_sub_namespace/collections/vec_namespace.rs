use std::{collections::HashSet, marker::PhantomData};

use delegate::delegate;

use paste::paste;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct VecNamespace<K, NS>
{

    namespace: NS,
    phantom_key: PhantomData<K>

}

impl<K, NS> VecNamespace<K, NS>
{

    pub fn new(namespace: NS) -> Self
    {

        Self
        {

            namespace,
            phantom_key: PhantomData::default()

        }

    }

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: K, value: bool) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &K, value: bool) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &K, value: bool) -> Option<bool>;

            pub async fn update_fn<R, FN: FnMut(&mut bool) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnMut(&K, &mut bool) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &K) -> Option<bool>;

            pub async fn read_fn<R, FN: Fn(&bool) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: Fn(&K, &bool) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &K) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: K, value: bool) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &K) -> async_graphql::Result<bool>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &K) -> Option<bool>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<K>
    {

        self.namespace.get_all_keys_clone().await

    }

}

