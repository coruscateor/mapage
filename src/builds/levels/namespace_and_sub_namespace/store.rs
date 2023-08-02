use corlib::has_one::*;

use crate::errors::invalid_operation;

use crate::types::async_graphql_values::*;

use paste::paste;

use std::collections::HashSet;

use corlib::impl_get_ref;

//use super::numeric_namespace::NumericNamespace;

#[cfg(any(feature = "all_types", feature = "bool"))]
use super::bool_namespace::BoolNamespace;

#[cfg(any(feature = "all_types", feature = "char"))]
use super::char_namespace::CharNamespace;

/*
#[cfg(any(feature = "all_types", feature = "f32", feature = "f64"))]
use super::numeric_namespace::NumericNamespace; //<f32>; //float_namespace::FloatNamespace;

#[cfg(any(feature = "all_types", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "isize"))]
use super::int_namespace::IntNamespace;
 */

 #[cfg(any(feature = "all_types", feature = "f32", feature = "f64", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "isize", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128", feature = "usize"))]
 use super::numeric_namespace::NumericNamespace; 

#[cfg(any(feature = "all_types", feature = "i128"))]
use crate::types::async_graphql_values::I128Scalar;

/*
#[cfg(any(feature = "all_types", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128", feature = "usize"))]
use super::uint_namespace::UintNamespace;
*/

#[cfg(any(feature = "all_types", feature = "u128"))]
use crate::types::async_graphql_values::U128Scalar;

#[cfg(any(feature = "all_types", feature = "String"))]
use super::string_namespace::StringNamespace;

#[cfg(any(feature = "all_types", feature = "Whatever"))]
use super::whatever_namespace::WhateverNamespace;

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
use super::selected_type_namespace::SelectedTypeNamespace;

pub struct Store 
{

    #[cfg(any(feature = "all_types", feature = "bool"))]
    bool_namespace: BoolNamespace,

    #[cfg(any(feature = "all_types", feature = "char"))]
    char_namespace: CharNamespace,

    #[cfg(any(feature = "all_types", feature = "f32"))]
    f32_namespace: NumericNamespace<f32>, //FloatNamespace<f32>,

    #[cfg(any(feature = "all_types", feature = "f64"))]
    f64_namespace: NumericNamespace<f64>, //FloatNamespace<f64>,

    #[cfg(any(feature = "all_types", feature = "i8"))]
    i8_namespace: NumericNamespace<i8>, //IntNamespace<i8>,

    #[cfg(any(feature = "all_types", feature = "i16"))]
    i16_namespace: NumericNamespace<i16>, ///IntNamespace<i16>,

    #[cfg(any(feature = "all_types", feature = "i32"))]
    i32_namespace: NumericNamespace<i32>, ///IntNamespace<i32>,

    #[cfg(any(feature = "all_types", feature = "i64"))]
    i64_namespace: NumericNamespace<i64>, ///IntNamespace<i64>,

    #[cfg(any(feature = "all_types", feature = "i128"))]
    i128_namespace: NumericNamespace<I128Scalar>, //IntNamespace<I128Scalar>,

    #[cfg(any(feature = "all_types", feature = "isize"))]
    isize_namespace: NumericNamespace<isize>, //IntNamespace<isize>,

    #[cfg(any(feature = "all_types", feature = "u8"))]
    u8_namespace: NumericNamespace<u8>, //UintNamespace<u8>,

    #[cfg(any(feature = "all_types", feature = "u16"))]
    u16_namespace: NumericNamespace<u16>, //UintNamespace<u16>,

    #[cfg(any(feature = "all_types", feature = "u32"))]
    u32_namespace: NumericNamespace<u32>, //UintNamespace<u32>,

    #[cfg(any(feature = "all_types", feature = "u64"))]
    u64_namespace: NumericNamespace<u64>, //UintNamespace<u64>,

    #[cfg(any(feature = "all_types", feature = "u128"))]
    u128_namespace: NumericNamespace<U128Scalar>, //UintNamespace<U128Scalar>,

    #[cfg(any(feature = "all_types", feature = "usize"))]
    usize_namespace: NumericNamespace<usize>, //UintNamespace<usize>,

    //vector namespace
    
    #[cfg(any(feature = "all_types", feature = "String"))]
    string_namespace: StringNamespace,

    //misc

    #[cfg(any(feature = "all_types", feature = "Whatever"))]
    whatever_namespace: WhateverNamespace,

    #[cfg(any(feature = "all_types", feature = "SelectedType"))]
    selected_type_namespace: SelectedTypeNamespace

}

impl Store
{

    pub fn new() -> Self
    {

        Self
        {

            #[cfg(any(feature = "all_types", feature = "bool"))]
            bool_namespace: BoolNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "char"))]
            char_namespace: CharNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "f32"))]
            f32_namespace: NumericNamespace::new(), //FloatNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "f64"))]
            f64_namespace: NumericNamespace::new(), //FloatNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i8"))]
            i8_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i16"))]
            i16_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i32"))]
            i32_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i64"))]
            i64_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i128"))]
            i128_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "isize"))]
            isize_namespace: NumericNamespace::new(), //IntNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u8"))]
            u8_namespace: NumericNamespace::new(), //UintNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u16"))]
            u16_namespace: NumericNamespace::new(), //UintNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u32"))]
            u32_namespace: NumericNamespace::new(), //UintNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u64"))]
            u64_namespace: NumericNamespace::new(), //UintNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u128"))]
            u128_namespace: NumericNamespace::new(), //UintNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "usize"))]
            usize_namespace: NumericNamespace::new(), //UintNamespace::new(),

            //vector namespace
            
            #[cfg(any(feature = "all_types", feature = "String"))]
            string_namespace: StringNamespace::new(),

            //misc

            #[cfg(any(feature = "all_types", feature = "Whatever"))]
            whatever_namespace: WhateverNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "SelectedType"))]
            selected_type_namespace: SelectedTypeNamespace::new()

        }

    }

    #[cfg(any(feature = "all_types", feature = "bool"))]
    impl_get_ref!(bool_namespace, BoolNamespace);

    #[cfg(any(feature = "all_types", feature = "char"))]
    impl_get_ref!(char_namespace, CharNamespace);

    #[cfg(any(feature = "all_types", feature = "f32"))]
    impl_get_ref!(f32_namespace, NumericNamespace<f32>); //FloatNamespace<f32>);

    #[cfg(any(feature = "all_types", feature = "f64"))]
    impl_get_ref!(f64_namespace, NumericNamespace<f64>); //FloatNamespace<f64>);

    #[cfg(any(feature = "all_types", feature = "i8"))]
    impl_get_ref!(i8_namespace, NumericNamespace<i8>); //IntNamespace<i8>);

    #[cfg(any(feature = "all_types", feature = "i16"))]
    impl_get_ref!(i16_namespace, NumericNamespace<i16>); //IntNamespace<i16>);

    #[cfg(any(feature = "all_types", feature = "i32"))]
    impl_get_ref!(i32_namespace, NumericNamespace<i32>); //IntNamespace<i32>);

    #[cfg(any(feature = "all_types", feature = "i64"))]
    impl_get_ref!(i64_namespace, NumericNamespace<i64>); //IntNamespace<i64>);

    #[cfg(any(feature = "all_types", feature = "i128"))]
    impl_get_ref!(i128_namespace, NumericNamespace<I128Scalar>); //IntNamespace<I128Scalar>);

    #[cfg(any(feature = "all_types", feature = "isize"))]
    impl_get_ref!(isize_namespace, NumericNamespace<isize>); //IntNamespace<isize>);

    #[cfg(any(feature = "all_types", feature = "u8"))]
    impl_get_ref!(u8_namespace, NumericNamespace<u8>); //UintNamespace<u8>);

    #[cfg(any(feature = "all_types", feature = "u16"))]
    impl_get_ref!(u16_namespace, NumericNamespace<u16>); //UintNamespace<u16>);

    #[cfg(any(feature = "all_types", feature = "u32"))]
    impl_get_ref!(u32_namespace, NumericNamespace<u32>); //UintNamespace<u32>);

    #[cfg(any(feature = "all_types", feature = "u64"))]
    impl_get_ref!(u64_namespace, NumericNamespace<u64>); //UintNamespace<u64>);

    #[cfg(any(feature = "all_types", feature = "u128"))]
    impl_get_ref!(u128_namespace, NumericNamespace<U128Scalar>); //UintNamespace<U128Scalar>);

    #[cfg(any(feature = "all_types", feature = "usize"))]
    impl_get_ref!(usize_namespace, NumericNamespace<usize>); //UintNamespace<usize>);

    #[cfg(any(feature = "all_types", feature = "string"))]
    impl_get_ref!(string_namespace, StringNamespace);

    #[cfg(any(feature = "all_types", feature = "whatever"))]
    impl_get_ref!(whatever_namespace, WhateverNamespace);

    #[cfg(any(feature = "all_types", feature = "SelectedType"))]
    impl_get_ref!(selected_type_namespace, SelectedTypeNamespace);
    
}


