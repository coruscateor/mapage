use std::{collections::HashSet, ops::{Add, AddAssign, Div, DivAssign, MulAssign, Mul, Neg, Rem, RemAssign, SubAssign, Sub, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Not}, fmt::{Display, LowerExp, UpperExp, Binary, LowerHex, Octal}, str::FromStr, iter::{Product, Sum}};

use std::hash::Hash;

use delegate::delegate;

use paste::paste;

use corlib::has_one::*;

use crate::{types::{ops::*, async_graphql_values::{I128Scalar, U128Scalar}}, impl_update_fn_op_method};

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace<K, T> = SCC_HashMapNamespace<K, T>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace<K, T> = DashMapNamespace<K, T>;

pub struct NumericNamespace<K, T>
    where K: 'static + Clone + Eq + Hash + Ord + Sync,
          T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp //+ Neg // + Sum<T> //+ Product<T>
{

    namespace: Namespace<K, T>

}

impl<K, T> NumericNamespace<K, T>
    where K: 'static + Clone + Eq + Hash + Ord + Sync,
          T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp //+ Neg //Sum<T> + //+ Product<T> 
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

            pub async fn insert(&self, key: K, value: T) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &K, value: T) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &K, value: T) -> Option<T>;

            pub async fn update_fn<R, FN: FnOnce(&mut T) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnOnce(&K, &mut T) -> async_graphql::Result<R>>(&self, key: &K, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &K) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &K) -> Option<T>;

            pub async fn read_fn<R, FN: FnOnce(&T) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;
            
            pub async fn read_kv_fn<R, FN: FnOnce(&K, &T) -> async_graphql::Result<R>>(&self, key: &K, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &K) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: K, value: T) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &K) -> async_graphql::Result<T>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &K) -> Option<T>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<K>
    {

        self.namespace.get_all_keys_clone().await

    }

    //Implement base traits

    //Ops

    impl_update_fn_op_method!(add, K, T, value: T);

    impl_update_fn_op_method!(add_self, K, T);

    impl_update_fn_op_method!(div, K, T, value: T);

    impl_update_fn_op_method!(div_self, K, T);

    impl_update_fn_op_method!(mul, K, T, value: T);

    impl_update_fn_op_method!(mul_self, K, T);

    impl_update_fn_op_method!(rem, K, T, value: T);

    impl_update_fn_op_method!(rem_self, K, T);

    impl_update_fn_op_method!(sub, K, T, value: T);

    impl_update_fn_op_method!(sub_self, K, T);

}

//Uints and ints - Neg

impl<K, T> NumericNamespace<K, T>
    where K: 'static + Clone + Eq + Hash + Ord + Sync,
          T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Binary + BitAnd<T> + BitAndAssign<T> + BitOr<T> + BitOrAssign<T> + BitXor<T> + BitXorAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + Hash + LowerExp + LowerHex + Mul<T> + MulAssign<T> + Octal + Ord + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Shl<T> + ShlAssign<T> + Shr<T> + ShrAssign<T> + Sub<T> + SubAssign<T> + ToString + UpperExp
{

    impl_update_fn_op_method!(bit_and, K, T, value: T);

    impl_update_fn_op_method!(bit_and_self, K, T);

    impl_update_fn_op_method!(bit_or, K, T, value: T);

    impl_update_fn_op_method!(bit_or_self, K, T);

    impl_update_fn_op_method!(bit_xor, K, T, value: T);

    impl_update_fn_op_method!(bit_xor_self, K, T);

    impl_update_fn_op_method!(shl, K, T, value: T);

    impl_update_fn_op_method!(shl_self, K, T);

    impl_update_fn_op_method!(shr, K, T, value: T);

    impl_update_fn_op_method!(shr_self, K, T);

}

//Base traits + Neg for floats and ints

#[derive(Default)]
pub struct I128ScalarHasOne();

impl HasOne<I128Scalar> for I128ScalarHasOne {
    
    fn one() -> I128Scalar
    {

        I128Scalar::new(1)

    }

}

#[derive(Default)]
pub struct U128ScalarHasOne();

impl HasOne<U128Scalar> for U128ScalarHasOne {
    
    fn one() -> U128Scalar
    {

        U128Scalar::new(1)

    }

}

impl<K> NumericNamespace<K, f32>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, f32);

    impl_update_fn_op_method!(inc, K, f32, F32HasOne);

    impl_update_fn_op_method!(dec, K, f32, F32HasOne);

}

impl<K> NumericNamespace<K, f64>
    where K: 'static + Clone + Eq + Hash + Ord + Sync    
{

    impl_update_fn_op_method!(neg, K, f64);

    impl_update_fn_op_method!(inc, K, f64, F64HasOne);

    impl_update_fn_op_method!(dec, K, f64, F64HasOne);

}

impl<K> NumericNamespace<K, i8>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, i8);

    impl_update_fn_op_method!(not, K, i8);

    impl_update_fn_op_method!(inc, K, i8, I8HasOne);

    impl_update_fn_op_method!(dec, K, i8, I8HasOne);

}

impl<K> NumericNamespace<K, i16>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, i16);

    impl_update_fn_op_method!(not, K, i16);

    impl_update_fn_op_method!(inc, K, i16, I16HasOne);

    impl_update_fn_op_method!(dec, K, i16, I16HasOne);

}

impl<K> NumericNamespace<K, i32>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, i32);

    impl_update_fn_op_method!(not, K, i32);

    impl_update_fn_op_method!(inc, K, i32, I32HasOne);

    impl_update_fn_op_method!(dec, K, i32, I32HasOne);

}

impl<K> NumericNamespace<K, i64>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, i64);

    impl_update_fn_op_method!(not, K, i64);

    impl_update_fn_op_method!(inc, K, i64, I64HasOne);

    impl_update_fn_op_method!(dec, K, i64, I64HasOne);

}

impl<K> NumericNamespace<K, I128Scalar>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, I128Scalar);

    impl_update_fn_op_method!(not, K, I128Scalar);

    impl_update_fn_op_method!(inc, K, I128Scalar, I128ScalarHasOne);

    impl_update_fn_op_method!(dec, K, I128Scalar, I128ScalarHasOne);

}

impl<K> NumericNamespace<K, isize>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(neg, K, isize);

    impl_update_fn_op_method!(not, K, isize);

    impl_update_fn_op_method!(inc, K, isize, ISizeHasOne);

    impl_update_fn_op_method!(dec, K, isize, ISizeHasOne);

}

//

impl<K> NumericNamespace<K, u8>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, u8);

    impl_update_fn_op_method!(inc, K, u8, U8HasOne);

    impl_update_fn_op_method!(dec, K, u8, U8HasOne);

}

impl<K> NumericNamespace<K, u16>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, u16);

    impl_update_fn_op_method!(inc, K, u16, U16HasOne);

    impl_update_fn_op_method!(dec, K, u16, U16HasOne);

}

impl<K> NumericNamespace<K, u32>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, u32);

    impl_update_fn_op_method!(inc, K, u32, U32HasOne);

    impl_update_fn_op_method!(dec, K, u32, U32HasOne);

}

impl<K> NumericNamespace<K, u64>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, u64);

    impl_update_fn_op_method!(inc, K, u64, U64HasOne);

    impl_update_fn_op_method!(dec, K, u64, U64HasOne);

}

impl<K> NumericNamespace<K, U128Scalar>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, U128Scalar);

    impl_update_fn_op_method!(inc, K, U128Scalar, U128ScalarHasOne);

    impl_update_fn_op_method!(dec, K, U128Scalar, U128ScalarHasOne);

}

impl<K> NumericNamespace<K, usize>
    where K: 'static + Clone + Eq + Hash + Ord + Sync
{

    impl_update_fn_op_method!(not, K, usize);

    impl_update_fn_op_method!(inc, K, usize, USizeHasOne);

    impl_update_fn_op_method!(dec, K, usize, USizeHasOne);

}

