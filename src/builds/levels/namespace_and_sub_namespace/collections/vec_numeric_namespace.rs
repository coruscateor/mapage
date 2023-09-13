use std::{collections::HashSet, ops::{Add, AddAssign, Div, DivAssign, MulAssign, Mul, Neg, Rem, RemAssign, SubAssign, Sub, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Not}, fmt::{Display, LowerExp, UpperExp, Binary, LowerHex, Octal}, str::FromStr, iter::{Product, Sum}};

use std::hash::Hash;

use delegate::delegate;

use crate::{types::ops::*, impl_update_fn_op_method};

crate::impl_vec_collection_fns_imports!();

#[cfg(feature = "scc_hashmap_namespaces")]
use super::super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace<K, T> = SCC_HashMapNamespace<K, T>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace<K, T> = DashMapNamespace<K, T>;

//K: 'static + Clone + Eq + Hash + Ord + Sync

pub struct VecNumericNamespace<K, T>
    where K: 'static + Clone + Eq + Hash + Ord + Sync,
          T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp
{

    namespace: Namespace<K, Vec<T>>

}

impl<K, T> VecNumericNamespace<K, T>
    where K: 'static + Clone + Eq + Hash + Ord + Sync,
          T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp
{

    pub fn new() -> Self
    {

        Self
        {

            namespace: Namespace::new()

        }

    }

    delegate! {
        to self.namespace {

            pub async fn insert(&self, key: K, value: Vec<T>) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &K, value: Vec<T>) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &K, value: Vec<T>) -> Option<Vec<T>>;

            pub async fn update_fn<R, FN: FnOnce(&mut Vec<T>) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&K, &mut Vec<T>) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &K) -> Option<Vec<T>>;

            pub async fn read_fn<R, FN: FnOnce(&Vec<T>) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;

            pub async fn read_kv_fn<R, FN: FnOnce(&K, &Vec<T>) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &K) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

            pub async fn upsert(&self, key: K, value: Vec<T>) -> async_graphql::Result<&'static str>;

            pub async fn read(&self, key: &K) -> async_graphql::Result<Vec<T>>;

            pub async fn try_read(&self, key: &K) -> Option<Vec<T>>;

            pub async fn get_all_keys(&self) -> HashSet<K>;

        }
    }

    //crate::impl_vec_methods_and_fns!(K, T);

    crate::impl_vec_fns_no_ord!(K, T);

    //Ops

}

//Uints and ints - Neg

//Ops




