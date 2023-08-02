use std::{collections::HashSet, ops::{Add, AddAssign, Div, DivAssign, MulAssign, Mul, Neg, Rem, RemAssign, SubAssign, Sub, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Not}, fmt::{Display, LowerExp, UpperExp, Binary, LowerHex, Octal}, str::FromStr, iter::{Product, Sum}};

use std::hash::Hash;

use delegate::delegate;

use paste::paste;

use corlib::has_one::*;

type KeyType = String;

use crate::{types::{ops::*, async_graphql_values::{I128Scalar, U128Scalar}}, impl_update_fn_op_method};

#[cfg(feature = "scc_hashmap_namespaces")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_namespaces")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; 

#[cfg(feature = "scc_hashmap_namespaces")]
type Namespace<T> = SCC_HashMapNamespace<KeyType, T>;

#[cfg(feature = "dashmap_namespaces")]
type Namespace<T> = DashMapNamespace<KeyType, T>;

//Floats - Neg

pub struct NumericNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp //+ Neg // + Sum<T> //+ Product<T>
{

    namespace: Namespace<T>

}

impl<T> NumericNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp //+ Neg //Sum<T> + //+ Product<T> 
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

            pub async fn insert(&self, key: KeyType, value: T) -> async_graphql::Result<&'static str>;

            pub async fn update(&self, key: &KeyType, value: T) -> async_graphql::Result<&'static str>;

            pub async fn try_replace(&self, key: &KeyType, value: T) -> Option<T>;

            pub async fn update_fn<R, FN: FnMut(&mut T) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;
                //where Fn(&mut T) -> async_graphql::Result<R>;

            pub async fn update_kv_fn<R, FN: FnMut(&KeyType, &mut T) -> async_graphql::Result<R>>(&self, key: &KeyType, updater: FN) -> async_graphql::Result<R>;

            pub async fn remove(&self, key: &KeyType) -> async_graphql::Result<&'static str>;

            pub async fn try_retrieve(&self, key: &KeyType) -> Option<T>;

            pub async fn read_fn<R, FN: Fn(&T) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;
                //Fn(&T) -> async_graphql::Result<R
            
            pub async fn read_kv_fn<R, FN: Fn(&KeyType, &T) -> async_graphql::Result<R>>(&self, key: &KeyType, reader: FN) -> async_graphql::Result<R>;

            pub async fn contains(&self, key: &KeyType) -> bool;

            pub async fn clear(&self) -> &'static str;

            pub async fn clear_and_get_len(&self) -> usize;

            pub async fn len(&self) -> usize;

            pub async fn is_empty(&self) -> bool;

            pub async fn capacity(&self) -> usize;

        }
    }

    pub async fn upsert(&self, key: KeyType, value: T) -> async_graphql::Result<&'static str>
    {

        self.namespace.upsert_copy(key, value).await

    }

    pub async fn read(&self, key: &KeyType) -> async_graphql::Result<T>
    {

        self.namespace.read_copy(key).await

    }

    pub async fn try_read(&self, key: &KeyType) -> Option<T>
    {

        self.namespace.try_read_copy(key).await

    }

    pub async fn get_all_keys(&self) -> HashSet<KeyType>
    {

        self.namespace.get_all_keys_clone().await

    }

    //Implement base traits

    //Ops

    impl_update_fn_op_method!(add, KeyType, T, value: T);

    impl_update_fn_op_method!(add_self, KeyType, T);

    impl_update_fn_op_method!(div, KeyType, T, value: T);

    impl_update_fn_op_method!(div_self, KeyType, T);

    impl_update_fn_op_method!(mul, KeyType, T, value: T);

    impl_update_fn_op_method!(mul_self, KeyType, T);

    impl_update_fn_op_method!(rem, KeyType, T, value: T);

    impl_update_fn_op_method!(rem_self, KeyType, T);

    impl_update_fn_op_method!(sub, KeyType, T, value: T);

    impl_update_fn_op_method!(sub_self, KeyType, T);

    //impl_update_fn_op_method!(inc, KeyType, T);

    //impl_update_fn_op_method!(dec, KeyType, T);



}

//Uints and ints - Neg

impl<T> NumericNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Binary + BitAnd<T> + BitAndAssign<T> + BitOr<T> + BitOrAssign<T> + BitXor<T> + BitXorAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + Hash + LowerExp + LowerHex + Mul<T> + MulAssign<T> + Octal + Ord + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Shl<T> + ShlAssign<T> + Shr<T> + ShrAssign<T> + Sub<T> + SubAssign<T> + ToString + UpperExp
{

    //_units_and_ints

    //impl_update_fn_op_method!(not, KeyType, T);

    impl_update_fn_op_method!(bit_and, KeyType, T, value: T);

    impl_update_fn_op_method!(bit_and_self, KeyType, T);

    impl_update_fn_op_method!(bit_or, KeyType, T, value: T);

    impl_update_fn_op_method!(bit_or_self, KeyType, T);

    impl_update_fn_op_method!(bit_xor, KeyType, T, value: T);

    impl_update_fn_op_method!(bit_xor_self, KeyType, T);

    impl_update_fn_op_method!(shl, KeyType, T, value: T);

    impl_update_fn_op_method!(shl_self, KeyType, T);

    impl_update_fn_op_method!(shr, KeyType, T, value: T);

    impl_update_fn_op_method!(shr_self, KeyType, T);

    /*
    impl_update_fn_op_method!(add, KeyType, T, value: T);

    impl_update_fn_op_method!(add_self, KeyType, T);

    impl_update_fn_op_method!(div, KeyType, T, value: T);

    impl_update_fn_op_method!(div_self, KeyType, T);

    impl_update_fn_op_method!(mul, KeyType, T, value: T);

    impl_update_fn_op_method!(mul_self, KeyType, T);

    impl_update_fn_op_method!(rem, KeyType, T, value: T);

    impl_update_fn_op_method!(rem_self, KeyType, T);

    impl_update_fn_op_method!(sub, KeyType, T, value: T);

    impl_update_fn_op_method!(sub_self, KeyType, T);

    impl_update_fn_op_method!(inc, KeyType, T);

    impl_update_fn_op_method!(dec, KeyType, T);
    */

    /*
    pub async fn placeholder(&self)
    {


    }
    */

}

//Base traits + Neg for floats and ints

/*
impl<T> NumericNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + LowerExp + Mul<T> + MulAssign<T> + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Sub<T> + SubAssign<T> + UpperExp + Neg //+ Not
{

    //_floars_and_ints

    /*
        type inside `async fn` body must be known in this context
        cannot infer typerustcClick for full compiler diagnostic
        numeric_namespace.rs(143, 5): Error originated from macro call here
        ops.rs(577, 55): the type is part of the `async fn` body because of this `await`
        the trait bound `T: std::ops::Not` is not satisfied
        the trait `std::ops::Not` is not implemented for `T`rustcClick for full compiler diagnostic
        numeric_namespace.rs(159, 5): Error originated from macro call here
        ops.rs(342, 18): required by a bound in `types::ops::not_fn`
        numeric_namespace.rs(154, 441): consider further restricting this bound: ` + std::ops::Not`
        type mismatch resolving `<T as Neg>::Output == T`
        expected type parameter `T`
        found associated type `<T as std::ops::Neg>::Output`rustcClick for full compiler diagnostic
        numeric_namespace.rs(217, 5): Error originated from macro call here
        numeric_namespace.rs(211, 6): this type parameter
        ops.rs(324, 22): required by a bound in `types::ops::neg_fn`
        numeric_namespace.rs(212, 259): consider further restricting this bound: `<Output = T>`
     */

    impl_update_fn_op_method!(neg, KeyType, T);

    /*
    pub async fn placeholder2(&self)
    {


    }
    */

}
*/

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

impl NumericNamespace<f32>
{

    impl_update_fn_op_method!(neg, KeyType, f32);

    impl_update_fn_op_method!(inc, KeyType, f32, F32HasOne);

    impl_update_fn_op_method!(dec, KeyType, f32, F32HasOne);

}

impl NumericNamespace<f64>
{

    impl_update_fn_op_method!(neg, KeyType, f64);

    impl_update_fn_op_method!(inc, KeyType, f64, F64HasOne);

    impl_update_fn_op_method!(dec, KeyType, f64, F64HasOne);

}

impl NumericNamespace<i8>
{

    impl_update_fn_op_method!(neg, KeyType, i8);

    impl_update_fn_op_method!(not, KeyType, i8);

    impl_update_fn_op_method!(inc, KeyType, i8, I8HasOne);

    impl_update_fn_op_method!(dec, KeyType, i8, I8HasOne);

}

impl NumericNamespace<i16>
{

    impl_update_fn_op_method!(neg, KeyType, i16);

    impl_update_fn_op_method!(not, KeyType, i16);

    impl_update_fn_op_method!(inc, KeyType, i16, I16HasOne);

    impl_update_fn_op_method!(dec, KeyType, i16, I16HasOne);

}

impl NumericNamespace<i32>
{

    impl_update_fn_op_method!(neg, KeyType, i32);

    impl_update_fn_op_method!(not, KeyType, i32);

    impl_update_fn_op_method!(inc, KeyType, i32, I32HasOne);

    impl_update_fn_op_method!(dec, KeyType, i32, I32HasOne);

}

impl NumericNamespace<i64>
{

    impl_update_fn_op_method!(neg, KeyType, i64);

    impl_update_fn_op_method!(not, KeyType, i64);

    impl_update_fn_op_method!(inc, KeyType, i64, I64HasOne);

    impl_update_fn_op_method!(dec, KeyType, i64, I64HasOne);

}

impl NumericNamespace<I128Scalar>
{

    impl_update_fn_op_method!(neg, KeyType, I128Scalar);

    impl_update_fn_op_method!(not, KeyType, I128Scalar);

    impl_update_fn_op_method!(inc, KeyType, I128Scalar, I128ScalarHasOne);

    impl_update_fn_op_method!(dec, KeyType, I128Scalar, I128ScalarHasOne);

}

impl NumericNamespace<isize>
{

    impl_update_fn_op_method!(neg, KeyType, isize);

    impl_update_fn_op_method!(not, KeyType, isize);

    impl_update_fn_op_method!(inc, KeyType, isize, ISizeHasOne);

    impl_update_fn_op_method!(dec, KeyType, isize, ISizeHasOne);

}

//

impl NumericNamespace<u8>
{

    impl_update_fn_op_method!(not, KeyType, u8);

    impl_update_fn_op_method!(inc, KeyType, u8, U8HasOne);

    impl_update_fn_op_method!(dec, KeyType, u8, U8HasOne);

}

impl NumericNamespace<u16>
{

    impl_update_fn_op_method!(not, KeyType, u16);

    impl_update_fn_op_method!(inc, KeyType, u16, U16HasOne);

    impl_update_fn_op_method!(dec, KeyType, u16, U16HasOne);

}

impl NumericNamespace<u32>
{

    impl_update_fn_op_method!(not, KeyType, u32);

    impl_update_fn_op_method!(inc, KeyType, u32, U32HasOne);

    impl_update_fn_op_method!(dec, KeyType, u32, U32HasOne);

}

impl NumericNamespace<u64>
{

    impl_update_fn_op_method!(not, KeyType, u64);

    impl_update_fn_op_method!(inc, KeyType, u64, U64HasOne);

    impl_update_fn_op_method!(dec, KeyType, u64, U64HasOne);

}

impl NumericNamespace<U128Scalar>
{

    impl_update_fn_op_method!(not, KeyType, U128Scalar);

    impl_update_fn_op_method!(inc, KeyType, U128Scalar, U128ScalarHasOne);

    impl_update_fn_op_method!(dec, KeyType, U128Scalar, U128ScalarHasOne);

}

impl NumericNamespace<usize>
{

    impl_update_fn_op_method!(not, KeyType, usize);

    impl_update_fn_op_method!(inc, KeyType, usize, USizeHasOne);

    impl_update_fn_op_method!(dec, KeyType, usize, USizeHasOne);

}


//Everything

/*
impl<T> NumericNamespace<T>
    where T: Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Binary + BitAnd<T> + BitAndAssign<T> + BitOr<T> + BitOrAssign<T> + BitXor<T> + BitXorAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + Hash + LowerExp + LowerHex + Mul<T> + MulAssign<T> + Octal + Ord + PartialEq<T> + PartialOrd<T> + Rem<T> + RemAssign<T> + Shl<T> + ShlAssign<T> + Shr<T> + ShrAssign<T> + Sub<T> + SubAssign<T> + ToString + UpperExp + Not
{

    //_ints

    pub async fn placeholder(&self)
    {


    }

}
*/
