use std::collections::HashSet;

use super::bool_namespace::BoolNamespace;

use corlib::impl_get_ref;

use paste::paste;

use delegate::delegate;

struct BoolNonManagedNamespace
{

    namespace: BoolNamespace

}

impl BoolNonManagedNamespace
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: BoolNamespace::new()

        }

    }

    impl_get_ref!(namespace, BoolNamespace);

    //String keys

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: String, value: bool) -> async_graphql::Result<&'static str>;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: String, value: bool) -> async_graphql::Result<&'static str>;

            pub async fn get_all_keys(&self) -> HashSet<String>;      

        }
    }

    pub async fn update(&self, key: String, value: bool) -> async_graphql::Result<&'static str>
    {

        self.namespace.update(&key, value).await

    }

    pub async fn try_replace(&self, key: String, value: bool) -> Option<bool>
    {

        self.namespace.try_replace(&key, value).await

    }

    pub async fn update_fn<R>(&self, key: String, updater: fn(&mut bool) -> async_graphql::Result<R>) -> async_graphql::Result<R>
    {

        self.namespace.update_fn(&key, updater).await

    }

    pub async fn remove(&self, key: String) -> async_graphql::Result<&'static str>
    {

        self.namespace.remove(&key).await

    }

    pub async fn try_retrieve(&self, key: String) -> Option<bool>
    {

        self.namespace.try_retrieve(&key).await

    }

    pub async fn read_fn<R>(&self, key: String, reader: fn(&bool) -> async_graphql::Result<R>) -> async_graphql::Result<R>
    {

        self.namespace.read_fn(&key, reader).await

    }

    pub async fn contains(&self, key: &String) -> bool
    {

        self.namespace.contains(&key).await

    }

    pub async fn read(&self, key: &String) -> async_graphql::Result<bool>
    {

        self.namespace.read(&key).await

    }

    pub async fn try_read(&self, key: &String) -> Option<bool>
    {

        self.namespace.try_read(&key).await

    }

}

