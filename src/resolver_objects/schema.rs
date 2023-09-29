use async_graphql::MergedObject;

#[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
use async_graphql::types::EmptyMutation;

use super::*;

/*
use super::
{
    BoolQuery,
    CharQuery,
    F32Query,
    F64Query,
    I8Query,
    I16Query,
    I32Query,
    I64Query,
    I128Query,
    ISizeQuery,
    MiscellaneousQuery,
    StringQuery,
    U8Query,
    U16Query,
    U32Query,
    U64Query,
    U128Query,
    USizeQuery,
    WhateverQuery,
    BoolMutation,
    CharMutation,
    F32Mutation,
    F64Mutation,
    I8Mutation,
    I16Mutation,
    I32Mutation,
    I64Mutation,
    I128Mutation,
    ISizeMutation,
    StringMutation,
    U8Mutation,
    U16Mutation,
    U32Mutation,
    U64Mutation,
    U128Mutation,
    USizeMutation,
    WhateverMutation,
    SelectedTypeQuery,
    SelectedTypeMutation,
    Cfgs
    
};
*/

use super::collections::vec::*;

#[derive(MergedObject, Default)]
pub struct QueryRoot
(

    //Configuration etc

    MiscellaneousQuery,
    Cfgs,

    //Fundamental Types
    
    BoolQuery,
    CharQuery,
    F32Query,
    F64Query,
    I8Query,
    I16Query,
    I32Query,
    I64Query,
    I128Query,
    ISizeQuery,
    StringQuery,
    U8Query,
    U16Query,
    U32Query,
    U64Query,
    U128Query,
    USizeQuery,
    WhateverQuery,
    SelectedTypeQuery,

    //Vecs

    VecBoolQuery,
    VecCharQuery,
    VecF32Query,
    VecF64Query,
    VecI8Query,
    VecI16Query,
    VecI32Query,
    VecI64Query,
    VecI128Query,
    VecISizeQuery,
    VecStringQuery,
    VecU8Query,
    VecU16Query,
    VecU32Query,
    VecU64Query,
    VecU128Query,
    VecUSizeQuery,
    VecWhateverQuery

);

#[cfg(any(feature = "store_aml", feature = "sub_store_aml"))]
#[derive(MergedObject, Default)]
pub struct MutationRoot
(

    //Fundamental Types

    BoolMutation,
    CharMutation,
    F32Mutation,
    F64Mutation, 
    I8Mutation,
    I16Mutation,
    I32Mutation,
    I64Mutation,
    I128Mutation,
    ISizeMutation,
    StringMutation,
    U8Mutation,
    U16Mutation,
    U32Mutation,
    U64Mutation, 
    U128Mutation,
    USizeMutation,
    WhateverMutation,
    SelectedTypeMutation,

    //Vecs

    VecBoolMutation,
    VecCharMutation,
    VecF32Mutation,
    VecF64Mutation, 
    VecI8Mutation,
    VecI16Mutation,
    VecI32Mutation,
    VecI64Mutation,
    VecI128Mutation,
    VecISizeMutation,
    VecStringMutation,
    VecU8Mutation,
    VecU16Mutation,
    VecU32Mutation,
    VecU64Mutation, 
    VecU128Mutation,
    VecUSizeMutation,
    VecWhateverMutation,
    //VecSelectedTypeMutation

);

#[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
pub type MutationRoot = EmptyMutation;



