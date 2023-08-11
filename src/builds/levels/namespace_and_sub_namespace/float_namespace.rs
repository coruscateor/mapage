use std::{collections::HashSet, ops::{Add, AddAssign, Div, DivAssign, MulAssign, Mul, Neg, Rem, RemAssign, SubAssign, Sub}, fmt::{Display, LowerExp, UpperExp}, str::FromStr, iter::{Product, Sum}};

use delegate::delegate;

type key_type = String;

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace<T> = SCC_HashMapNamespace<key_type, T>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace<T> = DashMapNamespace<key_type, T>;

//: 'static + Eq + Hash + Sync,

pub struct FloatNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + Neg + PartialEq<T> + PartialOrd<T> + Product<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + Sum<T> + UpperExp
{

    namespace: Namespace<T>

}

impl<T> FloatNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + Neg + PartialEq<T> + PartialOrd<T> + Product<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + Sum<T> + UpperExp
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: Namespace::new()

        }

    }

    //key_type keys

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: key_type, value: T) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &key_type, value: T) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &key_type, value: T) -> Option<T>;

            pub async fn update_fn<R>(&self, key: &key_type, updater: fn(&mut T) -> async_graphql::Result<R>) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &key_type) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &key_type) -> Option<T>;

            pub async fn read_fn<R>(&self, key: &key_type, reader: fn(&T) -> async_graphql::Result<R>) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &key_type) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: key_type, value: T) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &key_type) -> async_graphql::Result<T>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &key_type) -> Option<T>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<key_type>
    {

        self.namespace.get_all_keys_clone().await

    }

}

