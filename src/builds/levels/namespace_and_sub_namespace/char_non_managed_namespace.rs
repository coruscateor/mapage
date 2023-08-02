use std::collections::HashSet;

use super::char_namespace::CharNamespace;

use corlib::impl_get_ref;

use paste::paste;

use delegate::delegate;

struct CharNonManagedNamespace
{

    namespace: CharNamespace

}

impl CharNonManagedNamespace
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: CharNamespace::new()

        }

    }

    impl_get_ref!(namespace, CharNamespace);

    //String keys

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: String, value: char) -> async_graphql::Result<&'static str>;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: String, value: char) -> async_graphql::Result<&'static str>;

            pub async fn get_all_keys(&self) -> HashSet<String>;      

        }
    }

    pub async fn update(&self, key: String, value: char) -> async_graphql::Result<&'static str>
    {

        self.namespace.update(&key, value).await

    }

    pub async fn try_replace(&self, key: String, value: char) -> Option<char>
    {

        self.namespace.try_replace(&key, value).await

    }

    pub async fn update_fn<R>(&self, key: String, updater: fn(&mut char) -> async_graphql::Result<R>) -> async_graphql::Result<R>
    {

        self.namespace.update_fn(&key, updater).await

    }

    pub async fn remove(&self, key: String) -> async_graphql::Result<&'static str>
    {

        self.namespace.remove(&key).await

    }

    pub async fn try_retrieve(&self, key: String) -> Option<char>
    {

        self.namespace.try_retrieve(&key).await

    }

    pub async fn read_fn<R>(&self, key: String, reader: fn(&char) -> async_graphql::Result<R>) -> async_graphql::Result<R>
    {

        self.namespace.read_fn(&key, reader).await

    }

    pub async fn contains(&self, key: &String) -> bool
    {

        self.namespace.contains(&key).await

    }

    pub async fn read(&self, key: &String) -> async_graphql::Result<char>
    {

        self.namespace.read(&key).await

    }

    pub async fn try_read(&self, key: &String) -> Option<char>
    {

        self.namespace.try_read(&key).await

    }

}