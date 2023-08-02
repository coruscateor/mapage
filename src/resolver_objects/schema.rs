use async_graphql::{MergedObject, types::EmptyMutation};

use super::{BoolQuery, CharQuery, F32Query, F64Query, I8Query, I16Query, I32Query, I64Query, I128Query, ISizeQuery, MiscellaneousQuery, StringQuery, U8Query, U16Query, U32Query, U64Query, U128Query, /*UnitValueQuery,*/ USizeQuery, WhateverQuery,
    BoolMutation, CharMutation, F32Mutation, F64Mutation, I8Mutation, I16Mutation, I32Mutation, I64Mutation, I128Mutation, ISizeMutation, StringMutation, U8Mutation, U16Mutation, U32Mutation, U64Mutation, U128Mutation, /*UnitValueMutation,*/ USizeMutation, WhateverMutation, SelectedTypeQuery, SelectedTypeMutation, SelectedTypeQueryMisc, SelectedTypeIOQuery};

//MiscellaneousMutation,

#[derive(MergedObject, Default)]
pub struct QueryRoot(BoolQuery, CharQuery, F32Query, F64Query, I8Query, I16Query, I32Query, I64Query, I128Query, ISizeQuery, MiscellaneousQuery, StringQuery, U8Query, U16Query, U32Query, U64Query, U128Query, /*UnitValueQuery,*/ USizeQuery, WhateverQuery, SelectedTypeQuery, SelectedTypeQueryMisc, SelectedTypeIOQuery);

#[cfg(any(feature = "store_aml", feature = "sub_store_aml"))]
#[derive(MergedObject, Default)]
pub struct MutationRoot(BoolMutation, CharMutation, F32Mutation, F64Mutation, I8Mutation, I16Mutation, I32Mutation, I64Mutation, I128Mutation, ISizeMutation, StringMutation, U8Mutation, U16Mutation, U32Mutation, U64Mutation, U128Mutation, USizeMutation, WhateverMutation, SelectedTypeMutation);

#[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
pub type MutationRoot = EmptyMutation;

/*
cfg_if::cfg_if! {
    if #[cfg(any(feature = "all_types", feature = "bool", feature = "char", feature = "f32", feature = "f64", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "isize", feature = "String", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128", feature = "usize", feature = "Whatever", feature = "SelectedType", feature = "SelectedTypeOI"))] {

#[derive(MergedObject, Default)]
pub struct MutationRoot(BoolMutation, CharMutation, F32Mutation, F64Mutation, I8Mutation, I16Mutation, I32Mutation, I64Mutation, I128Mutation, ISizeMutation, StringMutation, U8Mutation, U16Mutation, U32Mutation, U64Mutation, U128Mutation, USizeMutation, WhateverMutation, SelectedTypeMutation);

/*UnitValueMutation,*/

pub type MutationType = MutationRoot; 

    }
    else
    {

pub type MutationType = EmptyMutation;

    }
}
*/

//pub 

//MiscellaneousMutation, 



