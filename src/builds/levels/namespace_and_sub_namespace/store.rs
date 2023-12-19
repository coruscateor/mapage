use paste::paste;

use corlib::impl_get_ref;

#[cfg(any(feature = "all_types", feature = "bool"))]
use super::bool_namespace::BoolNamespace;

#[cfg(any(feature = "all_types", feature = "char"))]
use super::char_namespace::CharNamespace;

#[cfg(any(feature = "all_types", feature = "f32", feature = "f64", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "isize", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128", feature = "usize"))]
use super::numeric_namespace::NumericNamespace; 

#[cfg(any(feature = "all_types", feature = "i128", feature = "Vec_i128"))]
use crate::types::async_graphql_values::I128Scalar;

#[cfg(any(feature = "all_types", feature = "u128", feature = "Vec_u128"))]
use crate::types::async_graphql_values::U128Scalar;

#[cfg(any(feature = "all_types", feature = "String"))]
use super::string_namespace::StringNamespace;

#[cfg(any(feature = "all_types", feature = "Whatever"))]
use super::whatever_namespace::WhateverNamespace;

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
use super::selected_type_namespace::SelectedTypeNamespace;

//Key Types

/* 
use crate::types::keys::{
    
    #[cfg(any(feature = "all_types", feature = "f32"))]
    F32KeyType, 
    #[cfg(any(feature = "all_types", feature = "f64"))]

    F64KeyType, I8KeyType, I16KeyType, I32KeyType, I64KeyType, I128KeyType, IsizeKeyType, U8KeyType, U16KeyType, U32KeyType, U64KeyType, U128KeyType, UsizeKeyType};
*/

#[cfg(any(feature = "all_types", feature = "f32"))]
use crate::types::keys::F32KeyType;

#[cfg(any(feature = "all_types", feature = "f64"))]
use crate::types::keys::F64KeyType;

#[cfg(any(feature = "all_types", feature = "i8"))]
use crate::types::keys::I8KeyType;

#[cfg(any(feature = "all_types", feature = "i16"))]
use crate::types::keys::I16KeyType;

#[cfg(any(feature = "all_types", feature = "i32"))]
use crate::types::keys::I32KeyType;

#[cfg(any(feature = "all_types", feature = "i64"))]
use crate::types::keys::I64KeyType;

#[cfg(any(feature = "all_types", feature = "i128"))]
use crate::types::keys::I128KeyType;

#[cfg(any(feature = "all_types", feature = "usize"))]
use crate::types::keys::ISizeKeyType;

#[cfg(any(feature = "all_types", feature = "u8"))]
use crate::types::keys::U8KeyType;

#[cfg(any(feature = "all_types", feature = "u16"))]
use crate::types::keys::U16KeyType;

#[cfg(any(feature = "all_types", feature = "u32"))]
use crate::types::keys::U32KeyType;

#[cfg(any(feature = "all_types", feature = "u64"))]
use crate::types::keys::U64KeyType;

#[cfg(any(feature = "all_types", feature = "u128"))]
use crate::types::keys::U128KeyType;

#[cfg(any(feature = "all_types", feature = "usize"))]
use crate::types::keys::USizeKeyType;

//vecs

#[cfg(any(feature = "all_types", feature = "Vec_bool"))]
use super::collections::vec_bool_namespace::VecBoolNamespace;

#[cfg(any(feature = "all_types", feature = "Vec_char"))]
use super::collections::vec_char_namespace::VecCharNamespace;

#[cfg(any(feature = "all_types", feature = "Vec_f32", feature = "Vec_f64", feature = "Vec_i8", feature = "Vec_i16", feature = "Vec_i32", feature = "Vec_i64", feature = "Vec_i128", feature = "Vec_isize", feature = "Vec_u8", feature = "Vec_u16", feature = "Vec_u32", feature = "Vec_u64", feature = "Vec_u128", feature = "Vec_usize"))]
use super::collections::vec_numeric_namespace::VecNumericNamespace;

#[cfg(any(feature = "all_types", feature = "VecString"))]
use super::collections::vec_string_namespace::VecStringNamespace;

#[cfg(any(feature = "all_types", feature = "VecWhatever"))]
use super::collections::vec_whatever_namespace::VecWhateverNamespace;

#[cfg(any(feature = "all_types", feature = "VecSelectedType"))]
use super::collections::vec_selected_type_namespace::VecSelectedTypeNamespace;

pub struct Store 
{

    #[cfg(any(feature = "all_types", feature = "bool"))]
    bool_namespace: BoolNamespace,

    #[cfg(any(feature = "all_types", feature = "char"))]
    char_namespace: CharNamespace,

    #[cfg(any(feature = "all_types", feature = "f32"))]
    f32_namespace: NumericNamespace<F32KeyType, f32>,

    #[cfg(any(feature = "all_types", feature = "f64"))]
    f64_namespace: NumericNamespace<F64KeyType, f64>,

    #[cfg(any(feature = "all_types", feature = "i8"))]
    i8_namespace: NumericNamespace<I8KeyType, i8>,

    #[cfg(any(feature = "all_types", feature = "i16"))]
    i16_namespace: NumericNamespace<I16KeyType, i16>,

    #[cfg(any(feature = "all_types", feature = "i32"))]
    i32_namespace: NumericNamespace<I32KeyType, i32>,

    #[cfg(any(feature = "all_types", feature = "i64"))]
    i64_namespace: NumericNamespace<I64KeyType, i64>,

    #[cfg(any(feature = "all_types", feature = "i128"))]
    i128_namespace: NumericNamespace<I128KeyType, I128Scalar>,

    #[cfg(any(feature = "all_types", feature = "isize"))]
    isize_namespace: NumericNamespace<ISizeKeyType, isize>,

    #[cfg(any(feature = "all_types", feature = "u8"))]
    u8_namespace: NumericNamespace<U8KeyType, u8>,

    #[cfg(any(feature = "all_types", feature = "u16"))]
    u16_namespace: NumericNamespace<U16KeyType, u16>,

    #[cfg(any(feature = "all_types", feature = "u32"))]
    u32_namespace: NumericNamespace<U32KeyType, u32>,

    #[cfg(any(feature = "all_types", feature = "u64"))]
    u64_namespace: NumericNamespace<U64KeyType, u64>,

    #[cfg(any(feature = "all_types", feature = "u128"))]
    u128_namespace: NumericNamespace<U128KeyType, U128Scalar>,

    #[cfg(any(feature = "all_types", feature = "usize"))]
    usize_namespace: NumericNamespace<USizeKeyType, usize>,
    
    #[cfg(any(feature = "all_types", feature = "String"))]
    string_namespace: StringNamespace,

    //misc

    #[cfg(any(feature = "all_types", feature = "Whatever"))]
    whatever_namespace: WhateverNamespace,

    #[cfg(any(feature = "all_types", feature = "SelectedType"))]
    selected_type_namespace: SelectedTypeNamespace,

    //vecs

    #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
    vec_bool_namespace: VecBoolNamespace,

    #[cfg(any(feature = "all_types", feature = "Vec_char"))]
    vec_char_namespace: VecCharNamespace,

    #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
    vec_f32_namespace: VecNumericNamespace<F32KeyType, f32>,

    #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
    vec_f64_namespace: VecNumericNamespace<F64KeyType, f64>,

    #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
    vec_i8_namespace: VecNumericNamespace<I8KeyType, i8>,

    #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
    vec_i16_namespace: VecNumericNamespace<I16KeyType, i16>,

    #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
    vec_i32_namespace: VecNumericNamespace<I32KeyType, i32>,

    #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
    vec_i64_namespace: VecNumericNamespace<I64KeyType, i64>,

    #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
    vec_i128_namespace: VecNumericNamespace<I128KeyType, I128Scalar>,

    #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
    vec_isize_namespace: VecNumericNamespace<ISizeKeyType, isize>,

    #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
    vec_u8_namespace: VecNumericNamespace<U8KeyType, u8>,

    #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
    vec_u16_namespace: VecNumericNamespace<U16KeyType, u16>,

    #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
    vec_u32_namespace: VecNumericNamespace<U32KeyType, u32>,

    #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
    vec_u64_namespace: VecNumericNamespace<U64KeyType, u64>,

    #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
    vec_u128_namespace: VecNumericNamespace<U128KeyType, U128Scalar>,

    #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
    vec_usize_namespace: VecNumericNamespace<USizeKeyType, usize>,
    
    #[cfg(any(feature = "all_types", feature = "VecString"))]
    vec_string_namespace: VecStringNamespace,

    //misc

    #[cfg(any(feature = "all_types", feature = "VecWhatever"))]
    vec_whatever_namespace: VecWhateverNamespace,

    #[cfg(any(feature = "all_types", feature = "VecSelectedType"))]
    vec_selected_type_namespace: VecSelectedTypeNamespace

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
            f32_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "f64"))]
            f64_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i8"))]
            i8_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i16"))]
            i16_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i32"))]
            i32_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i64"))]
            i64_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "i128"))]
            i128_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "isize"))]
            isize_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u8"))]
            u8_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u16"))]
            u16_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u32"))]
            u32_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u64"))]
            u64_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "u128"))]
            u128_namespace: NumericNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "usize"))]
            usize_namespace: NumericNamespace::new(),
            
            #[cfg(any(feature = "all_types", feature = "String"))]
            string_namespace: StringNamespace::new(),

            //misc

            #[cfg(any(feature = "all_types", feature = "Whatever"))]
            whatever_namespace: WhateverNamespace::new(),

            #[cfg(any(feature = "all_types", feature = "SelectedType"))]
            selected_type_namespace: SelectedTypeNamespace::new(),

            //vecs

            #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
            vec_bool_namespace: VecBoolNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_char"))]
            vec_char_namespace: VecCharNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
            vec_f32_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
            vec_f64_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
            vec_i8_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
            vec_i16_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
            vec_i32_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
            vec_i64_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
            vec_i128_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
            vec_isize_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
            vec_u8_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
            vec_u16_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
            vec_u32_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
            vec_u64_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
            vec_u128_namespace: VecNumericNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
            vec_usize_namespace: VecNumericNamespace::new(),
            
            #[cfg(any(feature = "all_types", feature = "VecString"))]
            vec_string_namespace: VecStringNamespace::new(),
        
            //misc
        
            #[cfg(any(feature = "all_types", feature = "VecWhatever"))]
            vec_whatever_namespace: VecWhateverNamespace::new(),
        
            #[cfg(any(feature = "all_types", feature = "VecSelectedType"))]
            vec_selected_type_namespace: VecSelectedTypeNamespace::new()
            
        }

    }

    #[cfg(any(feature = "all_types", feature = "bool"))]
    impl_get_ref!(bool_namespace, BoolNamespace);

    #[cfg(any(feature = "all_types", feature = "char"))]
    impl_get_ref!(char_namespace, CharNamespace);

    #[cfg(any(feature = "all_types", feature = "f32"))]
    impl_get_ref!(f32_namespace, NumericNamespace<F32KeyType, f32>);

    #[cfg(any(feature = "all_types", feature = "f64"))]
    impl_get_ref!(f64_namespace, NumericNamespace<F64KeyType, f64>);

    #[cfg(any(feature = "all_types", feature = "i8"))]
    impl_get_ref!(i8_namespace, NumericNamespace<I8KeyType, i8>);

    #[cfg(any(feature = "all_types", feature = "i16"))]
    impl_get_ref!(i16_namespace, NumericNamespace<I16KeyType, i16>);

    #[cfg(any(feature = "all_types", feature = "i32"))]
    impl_get_ref!(i32_namespace, NumericNamespace<I32KeyType, i32>);

    #[cfg(any(feature = "all_types", feature = "i64"))]
    impl_get_ref!(i64_namespace, NumericNamespace<I64KeyType, i64>);

    #[cfg(any(feature = "all_types", feature = "i128"))]
    impl_get_ref!(i128_namespace, NumericNamespace<I128KeyType, I128Scalar>);

    #[cfg(any(feature = "all_types", feature = "isize"))]
    impl_get_ref!(isize_namespace, NumericNamespace<ISizeKeyType, isize>);

    #[cfg(any(feature = "all_types", feature = "u8"))]
    impl_get_ref!(u8_namespace, NumericNamespace<U8KeyType, u8>);

    #[cfg(any(feature = "all_types", feature = "u16"))]
    impl_get_ref!(u16_namespace, NumericNamespace<U16KeyType, u16>);

    #[cfg(any(feature = "all_types", feature = "u32"))]
    impl_get_ref!(u32_namespace, NumericNamespace<U32KeyType, u32>);

    #[cfg(any(feature = "all_types", feature = "u64"))]
    impl_get_ref!(u64_namespace, NumericNamespace<U64KeyType, u64>);

    #[cfg(any(feature = "all_types", feature = "u128"))]
    impl_get_ref!(u128_namespace, NumericNamespace<U128KeyType, U128Scalar>);

    #[cfg(any(feature = "all_types", feature = "usize"))]
    impl_get_ref!(usize_namespace, NumericNamespace<USizeKeyType, usize>);

    #[cfg(any(feature = "all_types", feature = "String"))]
    impl_get_ref!(string_namespace, StringNamespace);

    #[cfg(any(feature = "all_types", feature = "Whatever"))]
    impl_get_ref!(whatever_namespace, WhateverNamespace);

    #[cfg(any(feature = "all_types", feature = "SelectedType"))]
    impl_get_ref!(selected_type_namespace, SelectedTypeNamespace);

    //vecs

    #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
    impl_get_ref!(vec_bool_namespace, VecBoolNamespace);

    #[cfg(any(feature = "all_types", feature = "Vec_char"))]
    impl_get_ref!(vec_char_namespace, VecCharNamespace);

    #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
    impl_get_ref!(vec_f32_namespace, VecNumericNamespace<F32KeyType, f32>);

    #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
    impl_get_ref!(vec_f64_namespace, VecNumericNamespace<F64KeyType, f64>);

    #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
    impl_get_ref!(vec_i8_namespace, VecNumericNamespace<I8KeyType, i8>);

    #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
    impl_get_ref!(vec_i16_namespace, VecNumericNamespace<I16KeyType, i16>);

    #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
    impl_get_ref!(vec_i32_namespace, VecNumericNamespace<I32KeyType, i32>);

    #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
    impl_get_ref!(vec_i64_namespace, VecNumericNamespace<I64KeyType, i64>);

    #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
    impl_get_ref!(vec_i128_namespace, VecNumericNamespace<I128KeyType, I128Scalar>);

    #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
    impl_get_ref!(vec_isize_namespace, VecNumericNamespace<ISizeKeyType, isize>);

    #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
    impl_get_ref!(vec_u8_namespace, VecNumericNamespace<U8KeyType, u8>);

    #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
    impl_get_ref!(vec_u16_namespace, VecNumericNamespace<U16KeyType, u16>);

    #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
    impl_get_ref!(vec_u32_namespace, VecNumericNamespace<U32KeyType, u32>);

    #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
    impl_get_ref!(vec_u64_namespace, VecNumericNamespace<U64KeyType, u64>);

    #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
    impl_get_ref!(vec_u128_namespace, VecNumericNamespace<U128KeyType, U128Scalar>);

    #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
    impl_get_ref!(vec_usize_namespace, VecNumericNamespace<USizeKeyType, usize>);

    #[cfg(any(feature = "all_types", feature = "VecString"))]
    impl_get_ref!(vec_string_namespace, VecStringNamespace);

    #[cfg(any(feature = "all_types", feature = "VecWhatever"))]
    impl_get_ref!(vec_whatever_namespace, VecWhateverNamespace);

    #[cfg(any(feature = "all_types", feature = "VecSelectedType"))]
    impl_get_ref!(vec_selected_type_namespace, VecSelectedTypeNamespace);
    
}


