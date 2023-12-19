use std::collections::HashSet;

use async_graphql::{Object, Context};

use crate::types::async_graphql_values::I128Scalar;
use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

use crate::StoreType;

#[cfg(any(feature = "all_types", feature = "Vec_i128"))]
type KeyType = crate::types::keys::VecI128KeyType;

#[derive(Default)]
pub struct VecI128Query;

#[cfg(any(feature = "all_types", feature = "Vec_i128"))]
#[Object]
impl VecI128Query
{

    pub async fn vec_i128_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read, key)

    }

    pub async fn vec_i128_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, try_read, key)

    }

    pub async fn vec_i128_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, contains, key)

    }

    pub async fn vec_i128_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, len)

    }

    pub async fn vec_i128_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, is_empty)

    }

    pub async fn vec_i128_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, capacity)

    }

    pub async fn vec_i128_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, get_all_keys)

    }

    //read fns

    pub async fn vec_i128_read_capacity(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_capacity, key)

    }

    pub async fn vec_i128_read_len(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_len, key)

    }

    pub async fn vec_i128_read_is_empty(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_is_empty, key)

    }

    pub async fn vec_i128_read_first(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_first, key)

    }

    pub async fn vec_i128_read_last(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_last, key)

    }

    pub async fn vec_i128_read_binary_search(&self, ctx: &Context<'_>, key: KeyType, x: I128Scalar) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_binary_search, key, x)

    }

    pub async fn vec_i128_read_index(&self, ctx: &Context<'_>, key: KeyType, index: usize) -> async_graphql::Result<I128Scalar>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, read_index, key, index)

    }
    
}

#[cfg(not(any(feature = "all_types", feature = "Vec_i128")))]
#[Object]
impl VecI128Query 
{

    #[graphql(visible = false)]
    pub async fn vec_i128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct VecI128Mutation;

#[cfg(any(feature = "all_types", feature = "Vec_i128"))]
#[Object]
impl VecI128Mutation
{
    
    pub async fn vec_i128_insert(&self, ctx: &Context<'_>, key: KeyType, value: Vec<I128Scalar>) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_i128_namespace_ref, insert, key, value)

    }

    pub async fn vec_i128_update(&self, ctx: &Context<'_>, key: KeyType, value: Vec<I128Scalar>) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update, key, value)

    }

    pub async fn vec_i128_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: Vec<I128Scalar>) -> Option<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, try_replace, key, value)

    }

    pub async fn vec_i128_upsert(&self, ctx: &Context<'_>, key: KeyType, value: Vec<I128Scalar>) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_i128_namespace_ref, upsert, key, value)

    }

    pub async fn vec_i128_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, remove, key)

    }

    pub async fn vec_i128_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, try_retrieve, key)

    }

    pub async fn vec_i128_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, clear)

    }

    pub async fn vec_i128_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_i128_namespace_ref, clear_and_get_len)

    }

    //insert fns - Constructors

    pub async fn vec_i128_insert_new(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_i128_namespace_ref, insert_new, key)

    }

    pub async fn vec_i128_insert_with_capacity(&self, ctx: &Context<'_>, key: KeyType, capacity: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_i128_namespace_ref, insert_with_capacity, key, capacity)

    }

    pub async fn vec_i128_insert_with_no_capacity(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_i128_namespace_ref, insert_with_no_capacity, key)

    }

    //update fns
    
    pub async fn vec_i128_update_index_mut(&self, ctx: &Context<'_>, key: KeyType, index: usize, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_index_mut, key, index, value)

    }

    pub async fn vec_i128_update_reserve(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_reserve, key, additional)

    }

    pub async fn vec_i128_update_reserve_exact(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_reserve_exact, key, additional)

    }

    pub async fn vec_i128_update_try_reserve(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_reserve_exact, key, additional)
        
    }

    pub async fn vec_i128_update_try_reserve_exact(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_try_reserve_exact, key, additional)
        
    }

    pub async fn vec_i128_update_shrink_to_fit(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_shrink_to_fit, key)

    }

    pub async fn vec_i128_update_shrink_to(&self, ctx: &Context<'_>, key: KeyType, min_capacity: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_shrink_to, key, min_capacity)
        
    }

    pub async fn vec_i128_update_truncate(&self, ctx: &Context<'_>, key: KeyType, len: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_shrink_to, key, len)
        
    }

    pub async fn vec_i128_update_insert(&self, ctx: &Context<'_>, key: KeyType, index: usize, element: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_insert, key, index, element)
        
    }

    pub async fn vec_i128_update_push(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_push, key, value)
        
    }

    pub async fn vec_i128_update_pop(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_pop, key)
        
    }

    pub async fn vec_i128_update_append(&self, ctx: &Context<'_>, key: KeyType, value: Vec<I128Scalar>) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_append, key, value)
        
    }

    pub async fn vec_i128_update_clear(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_clear, key)
        
    }

    pub async fn vec_i128_update_split_off(&self, ctx: &Context<'_>, key: KeyType, at: usize) -> async_graphql::Result<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_split_off, key, at)
        
    }

    pub async fn vec_i128_update_resize(&self, ctx: &Context<'_>, key: KeyType, new_len: usize, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_resize, key, new_len, value)
        
    }

    pub async fn vec_i128_update_dedup(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_dedup, key)
        
    }

    pub async fn vec_i128_update_sort_unstable(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_sort_unstable, key)
        
    }

    pub async fn vec_i128_update_rotate_left(&self, ctx: &Context<'_>, key: KeyType, mid: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_rotate_left, key, mid)
        
    }

    pub async fn vec_i128_update_rotate_right(&self, ctx: &Context<'_>, key: KeyType, mid: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_rotate_right, key, mid)
        
    }

    pub async fn vec_i128_update_fill(&self, ctx: &Context<'_>, key: KeyType, value: I128Scalar) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_fill, key, value)
        
    }

    pub async fn vec_i128_update_sort(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_sort, key)
        
    }

    pub async fn vec_i128_update_swap(&self, ctx: &Context<'_>, key: KeyType, a: usize, b: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_swap, key, a, b)
        
    }

    pub async fn vec_i128_update_retrieve_contents(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Vec<I128Scalar>>
    {

        call_store_method!(ctx, get_vec_i128_namespace_ref, update_retrieve_contents, key)
        
    }

}

#[cfg(not(any(feature = "all_types", feature = "Vec_i128")))]
#[Object]
impl VecI128Mutation
{

    #[graphql(visible = false)]
    pub async fn vec_i128_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
