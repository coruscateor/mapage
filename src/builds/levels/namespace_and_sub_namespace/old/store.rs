//use scc::HashMap;
//use super::mutex::{mutex_bool_value::MutexBoolValue, mutex_char_value::MutexCharValue, mutex_numeric_value::MutexNumericValue, mutex_string_value::MutexStringValue};

use corlib::has_one::*;

use crate::errors::invalid_operation;

//use crate::types::UnitValue;

use crate::types::async_graphql_values::*;

use paste::paste;

use crate::{impl_store_namespace_all_methods_key_clone_value_copy, impl_store_namespace_all_methods_key_clone_value_clone, };

use crate::types::ops::*;

use std::collections::HashSet;

//use crate::types::scalar_type_functions::{bool_bit_and, bool_bit_or, bool_bit_xor, bool_not};

#[cfg(feature = "scc_hashmap_string")]
use super::scc_crate::hashmap_namespace::HashMapNamespace as SCC_HashMapNamespace;

#[cfg(feature = "dashmap_string")]
use super::dashmap_crate::dashmap_namespace::DashMapNamespace; // as Dashmap_Namespace;

//type Namespace<K, V> = HashMapNamespace<K, V>;

//bool

#[cfg(feature = "scc_hashmap_string")]
type BoolNamespace = SCC_HashMapNamespace<String, bool>;

#[cfg(feature = "dashmap_string")]
type BoolNamespace = DashMapNamespace<String, bool>;

//char

#[cfg(feature = "scc_hashmap_string")]
type CharNamespace = SCC_HashMapNamespace<String, char>;

#[cfg(feature = "dashmap_string")]
type CharNamespace = DashMapNamespace<String, char>;

//f32

#[cfg(feature = "scc_hashmap_string")]
type F32Namespace = SCC_HashMapNamespace<String, f32>;

#[cfg(feature = "dashmap_string")]
type F32Namespace = DashMapNamespace<String, f32>;

//f64

#[cfg(feature = "scc_hashmap_string")]
type F64Namespace = SCC_HashMapNamespace<String, f64>;

#[cfg(feature = "dashmap_string")]
type F64Namespace = DashMapNamespace<String, f64>;

//i8

#[cfg(feature = "scc_hashmap_string")]
type I8Namespace = SCC_HashMapNamespace<String, i8>;

#[cfg(feature = "dashmap_string")]
type I8Namespace = DashMapNamespace<String, i8>;

//i16

#[cfg(feature = "scc_hashmap_string")]
type I16Namespace = SCC_HashMapNamespace<String, i16>;

#[cfg(feature = "dashmap_string")]
type I16Namespace = DashMapNamespace<String, i16>;

//i32

#[cfg(feature = "scc_hashmap_string")]
type I32Namespace = SCC_HashMapNamespace<String, i32>;

#[cfg(feature = "dashmap_string")]
type I32Namespace = DashMapNamespace<String, i32>;

//i64

#[cfg(feature = "scc_hashmap_string")]
type I64Namespace = SCC_HashMapNamespace<String, i64>;

#[cfg(feature = "dashmap_string")]
type I64Namespace = DashMapNamespace<String, i64>;

//i128

#[cfg(feature = "scc_hashmap_string")]
type I128Namespace = SCC_HashMapNamespace<String, I128Scalar>;

#[cfg(feature = "dashmap_string")]
type I128Namespace = DashMapNamespace<String, I128Scalar>;

//isize

#[cfg(feature = "scc_hashmap_string")]
type IsizeNamespace = SCC_HashMapNamespace<String, isize>;

#[cfg(feature = "dashmap_string")]
type IsizeNamespace = DashMapNamespace<String, isize>;

//u8

#[cfg(feature = "scc_hashmap_string")]
type U8Namespace = SCC_HashMapNamespace<String, u8>;

#[cfg(feature = "dashmap_string")]
type U8Namespace = DashMapNamespace<String, u8>;

//u16

#[cfg(feature = "scc_hashmap_string")]
type U16Namespace = SCC_HashMapNamespace<String, u16>;

#[cfg(feature = "dashmap_string")]
type U16Namespace = DashMapNamespace<String, u16>;

//u32

#[cfg(feature = "scc_hashmap_string")]
type U32Namespace = SCC_HashMapNamespace<String, u32>;

#[cfg(feature = "dashmap_string")]
type U32Namespace = DashMapNamespace<String, u32>;

//u64

#[cfg(feature = "scc_hashmap_string")]
type U64Namespace = SCC_HashMapNamespace<String, u64>;

#[cfg(feature = "dashmap_string")]
type U64Namespace = DashMapNamespace<String, u64>;

//128

#[cfg(feature = "scc_hashmap_string")]
type U128Namespace = SCC_HashMapNamespace<String, U128Scalar>;

#[cfg(feature = "dashmap_string")]
type U128Namespace = DashMapNamespace<String, U128Scalar>;

//usize

#[cfg(feature = "scc_hashmap_string")]
type UsizeNamespace = SCC_HashMapNamespace<String, usize>;

#[cfg(feature = "dashmap_string")]
type UsizeNamespace = DashMapNamespace<String, usize>;

//String

#[cfg(feature = "scc_hashmap_string")]
type StringNamespace = SCC_HashMapNamespace<String, String>;

#[cfg(feature = "dashmap_string")]
type StringNamespace = DashMapNamespace<String, String>;

//Whatever

#[cfg(feature = "scc_hashmap_string")]
type WhateverNamespace = SCC_HashMapNamespace<String, Whatever>;

#[cfg(feature = "dashmap_string")]
type WhateverNamespace = DashMapNamespace<String, Whatever>;

pub struct Store 
{

    //#[cfg(any(all_types, bool))]
    #[cfg(any(feature = "all_types", feature = "bool"))]
    bool_values: BoolNamespace,

    #[cfg(any(feature = "all_types", feature = "char"))]
    char_values: CharNamespace,

    #[cfg(any(feature = "all_types", feature = "f32"))]
    f32_values: F32Namespace, //MutexNumericValue<f32, F32HasOne>>,

    #[cfg(any(feature = "all_types", feature = "f64"))]
    f64_values: F64Namespace,

    #[cfg(any(feature = "all_types", feature = "i8"))]
    i8_values: I8Namespace,

    #[cfg(any(feature = "all_types", feature = "i16"))]
    i16_values: I16Namespace,

    #[cfg(any(feature = "all_types", feature = "i32"))]
    i32_values: I32Namespace,

    #[cfg(any(feature = "all_types", feature = "i64"))]
    i64_values: I64Namespace,
    //i128_values: HashMap<String, i128>,

    #[cfg(any(feature = "all_types", feature = "i128"))]
    i128_values: I128Namespace,

    #[cfg(any(feature = "all_types", feature = "isize"))]
    isize_values: IsizeNamespace,

    #[cfg(any(feature = "all_types", feature = "u8"))]
    u8_values: U8Namespace,

    #[cfg(any(feature = "all_types", feature = "u16"))]
    u16_values: U16Namespace,

    #[cfg(any(feature = "all_types", feature = "u32"))]
    u32_values: U32Namespace,

    #[cfg(any(feature = "all_types", feature = "u64"))]
    u64_values: U64Namespace,
    //u128_values: HashMap<String, u128>,

    #[cfg(any(feature = "all_types", feature = "u128"))]
    u128_values: U128Namespace,
    //unit_values: Namespace<String, UnitValue>,

    #[cfg(any(feature = "all_types", feature = "usize"))]
    usize_values: UsizeNamespace,

    //vector values
    
    #[cfg(any(feature = "all_types", feature = "String"))]
    string_values: StringNamespace,

    //misc

    #[cfg(any(feature = "all_types", feature = "Whatever"))]
    whatever_values: WhateverNamespace

}

impl Store
{

    pub fn new() -> Self
    {

        Self
        {

            //#[cfg(any(all_types, bool))]
            #[cfg(any(feature = "all_types", feature = "bool"))]
            bool_values: BoolNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "char"))]
            char_values: CharNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "f32"))]
            f32_values: F32Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "f64"))]
            f64_values: F64Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "i8"))]
            i8_values: I8Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "i16"))]
            i16_values: I16Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "i32"))]
            i32_values: I32Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "i64"))]
            i64_values: I64Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "i128"))]
            i128_values: I128Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "isize"))]
            isize_values: IsizeNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u8"))]
            u8_values: U8Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "u16"))]
            u16_values: U16Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "u32"))]
            u32_values: U32Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "u64"))]
            u64_values: U64Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "u128"))]
            u128_values: U128Namespace::new(),
            //unit_values: Namespace::new(),

            #[cfg(any(feature = "all_types", feature = "usize"))]
            usize_values: UsizeNamespace::new(),

            //vector values
            
            #[cfg(any(feature = "all_types", feature = "String"))]
            string_values: StringNamespace::new(),

            //misc

            #[cfg(any(feature = "all_types", feature = "Whatever"))]
            whatever_values: WhateverNamespace::new()

        }

    }

    cfg_if::cfg_if! {
        //if #[cfg(any(all_types, bool))]  {
        if #[cfg(any(feature = "all_types", feature = "bool"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(bool, bool_values, String, bool);

    //Ops

    crate::impl_store_namespace_update_fn!(bool_not, bool_values, String, not, bool);

    //crate::impl_scc_hashmap_update_fn_param!(bool_bit_and, bool_values, String, bool, bit_and, bool);

    //($label:ident, $field:ident, $key_type:ty, $updater_fn_name:ident, $return_type:ty, $type_param:ty)

    crate::impl_store_namespace_update_fn_param!(bool_bit_and, bool_values, String, bit_and, bool, bool);

    //crate::impl_scc_hashmap_update_fn_param!(bool_bit_or, bool_values, String, bool, bit_or, bool);

    crate::impl_store_namespace_update_fn_param!(bool_bit_or, bool_values, String, bit_or, bool, bool);

    //crate::impl_scc_hashmap_update_fn_param!(bool_bit_xor, bool_values, String, bool, bit_xor, bool);

    crate::impl_store_namespace_update_fn_param!(bool_bit_xor, bool_values, String, bit_xor, bool, bool);

    crate::impl_store_namespace_update_fn!(bool_bit_and_self, bool_values, String, bit_and_self, bool);

    crate::impl_store_namespace_update_fn!(bool_bit_or_self, bool_values, String, bit_or_self, bool);

    crate::impl_store_namespace_update_fn!(bool_bit_xor_self, bool_values, String, bit_xor_self, bool);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "char"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(char, char_values, String, char);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "f32"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(f32, f32_values, String, f32);

    crate::impl_store_namespace_floating_point_ops!(f32, String, f32, f32, inc_f32, dec_f32);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "f64"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(f64, f64_values, String, f64);

    crate::impl_store_namespace_floating_point_ops!(f64, String, f64, f64, inc_f64, dec_f64);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "i8"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(i8, i8_values, String, i8);

    crate::impl_store_namespace_signed_integer_ops!(i8, String, i8, i8, inc_i8, dec_i8);
    
        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "i16"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(i16, i16_values, String, i16);

    crate::impl_store_namespace_signed_integer_ops!(i16, String, i16, i16, inc_i16, dec_i16);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "i32"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(i32, i32_values, String, i32);

    crate::impl_store_namespace_signed_integer_ops!(i32, String, i32, i32, inc_i32, dec_i32);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "i64"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(i64, i64_values, String, i64);

    crate::impl_store_namespace_signed_integer_ops!(i64, String, i64, i64, inc_i64, dec_i64);

        }
    }

    //

    //impl_scc_hashmap_all_methods_copy!(i128, i128_values, i128);

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "i128"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(i128, i128_values, String, I128Scalar);

        }
    }

    //I128Scalar needs traits implemented

    //crate::impl_store_namespace_integer_ops!(i128, String, I128Scalar, I128Scalar);

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "isize"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(isize, isize_values, String, isize);

    crate::impl_store_namespace_signed_integer_ops!(isize, String, isize, isize, inc_isize, dec_isize);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "u8"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(u8, u8_values, String, u8);

    crate::impl_store_namespace_unsigned_integer_ops!(u8, String, u8, u8, inc_u8, dec_u8);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "u16"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(u16, u16_values, String, u16);

    crate::impl_store_namespace_unsigned_integer_ops!(u16, String, u16, u16, inc_u16, dec_u16);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "u32"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(u32, u32_values, String, u32);

    crate::impl_store_namespace_unsigned_integer_ops!(u32, String, u32, u32, inc_u32, dec_u32);

        }
    }

    //

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "u64"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(u64, u64_values, String, u64);

    crate::impl_store_namespace_unsigned_integer_ops!(u64, String, u64, u64, inc_u64, dec_u64);

        }
    }

    //

    //impl_scc_hashmap_all_methods_copy!(u128, u128_values, u128);

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "u128"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(u128, u128_values, String, U128Scalar);

        }
    }

    //U128Scalar needs traits implemented

    //

    //crate::impl_store_namespace_all_methods_key_clone_value_copy!(unit_value, unit_values, String, UnitValue);

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "usize"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_copy!(usize, usize_values, String, usize);

    crate::impl_store_namespace_unsigned_integer_ops!(usize, String, usize, usize, inc_usize, dec_usize);

        }
    }

    //

    //vector values

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "String"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_clone!(string, string_values, String, String);

        }
    }

    //methods

    //macros

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "all_types", feature = "Whatever"))] {

    crate::impl_store_namespace_all_methods_key_clone_value_clone!(whatever, whatever_values, String, Whatever);

        }
    }

    //methods

    //Functions etc

    /*
    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    //impl_scc_hashmap_update_function_call!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    //impl_scc_hashmap_upsert_function_call!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_or, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_xor, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_not, bool);
    */

}


