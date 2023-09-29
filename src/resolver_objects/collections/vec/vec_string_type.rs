use std::collections::HashSet;

use async_graphql::{Object, Context};

use crate::{call_store_method, call_store_method_no_key, call_store_method_move_key};

use crate::StoreType;

type KeyType = crate::types::keys::VecStringKeyType;

#[derive(Default)]
pub struct VecStringQuery;

#[cfg(any(feature = "all_types", feature = "Vec_String"))]
#[Object]
impl VecStringQuery
{

    pub async fn vec_string_read(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read, key)

    }

    pub async fn vec_string_try_read(&self, ctx: &Context<'_>, key: KeyType) -> Option<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, try_read, key)

    }

    pub async fn vec_string_contains(&self, ctx: &Context<'_>, key: KeyType) -> bool
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, contains, key)

    }

    pub async fn vec_string_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, len)

    }

    pub async fn vec_string_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, is_empty)

    }

    pub async fn vec_string_capacity(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, capacity)

    }

    pub async fn vec_string_get_all_keys(&self, ctx: &Context<'_>) -> HashSet<KeyType>
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, get_all_keys)

    }

    //read fns

    pub async fn vec_string_read_capacity(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_capacity, key)

    }

    pub async fn vec_string_read_len(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_len, key)

    }

    pub async fn vec_string_read_is_empty(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<bool>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_is_empty, key)

    }

    pub async fn vec_string_read_first(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_first, key)

    }

    pub async fn vec_string_read_last(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_last, key)

    }

    pub async fn vec_string_read_binary_search(&self, ctx: &Context<'_>, key: KeyType, x: String) -> async_graphql::Result<usize>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_binary_search, key, x)

    }

    pub async fn vec_string_read_index(&self, ctx: &Context<'_>, key: KeyType, index: usize) -> async_graphql::Result<String>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, read_index, key, index)

    }
    
}

#[cfg(not(any(feature = "all_types", feature = "Vec_String")))]
#[Object]
impl VecStringQuery 
{

    #[graphql(visible = false)]
    pub async fn vec_string_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}

#[derive(Default)]
pub struct VecStringMutation;

#[cfg(any(feature = "all_types", feature = "Vec_String"))]
#[Object]
impl VecStringMutation
{
    
    pub async fn vec_string_insert(&self, ctx: &Context<'_>, key: KeyType, value: Vec<String>) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_string_namespace_ref, insert, key, value)

    }

    pub async fn vec_string_update(&self, ctx: &Context<'_>, key: KeyType, value: Vec<String>) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update, key, value)

    }

    pub async fn vec_string_try_replace(&self, ctx: &Context<'_>, key: KeyType, value: Vec<String>) -> Option<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, try_replace, key, value)

    }

    pub async fn vec_string_upsert(&self, ctx: &Context<'_>, key: KeyType, value: Vec<String>) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_string_namespace_ref, upsert, key, value)

    }

    pub async fn vec_string_remove(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, remove, key)

    }

    pub async fn vec_string_try_retrieve(&self, ctx: &Context<'_>, key: KeyType) -> Option<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, try_retrieve, key)

    }

    pub async fn vec_string_clear(&self, ctx: &Context<'_>) -> &'static str
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, clear)

    }

    pub async fn vec_string_clear_and_get_len(&self, ctx: &Context<'_>) -> usize
    {

        call_store_method_no_key!(ctx, get_vec_string_namespace_ref, clear_and_get_len)

    }

    //insert fns - Constructors

    pub async fn vec_string_insert_new(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_string_namespace_ref, insert_new, key)

    }

    pub async fn vec_string_insert_with_capacity(&self, ctx: &Context<'_>, key: KeyType, capacity: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_string_namespace_ref, insert_with_capacity, key, capacity)

    }

    pub async fn vec_string_insert_with_no_capacity(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method_move_key!(ctx, get_vec_string_namespace_ref, insert_with_no_capacity, key)

    }

    //update fns
    
    pub async fn vec_string_update_index_mut(&self, ctx: &Context<'_>, key: KeyType, index: usize, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_index_mut, key, index, value)

    }

    pub async fn vec_string_update_reserve(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_reserve, key, additional)

    }

    pub async fn vec_string_update_reserve_exact(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_reserve_exact, key, additional)

    }

    pub async fn vec_string_update_try_reserve(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_reserve_exact, key, additional)
        
    }

    pub async fn vec_string_update_try_reserve_exact(&self, ctx: &Context<'_>, key: KeyType, additional: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_try_reserve_exact, key, additional)
        
    }

    pub async fn vec_string_update_shrink_to_fit(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_shrink_to_fit, key)

    }

    pub async fn vec_string_update_shrink_to(&self, ctx: &Context<'_>, key: KeyType, min_capacity: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_shrink_to, key, min_capacity)
        
    }

    pub async fn vec_string_update_truncate(&self, ctx: &Context<'_>, key: KeyType, len: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_shrink_to, key, len)
        
    }

    pub async fn vec_string_update_insert(&self, ctx: &Context<'_>, key: KeyType, index: usize, element: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_insert, key, index, element)
        
    }

    pub async fn vec_string_update_push(&self, ctx: &Context<'_>, key: KeyType, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_push, key, value)
        
    }

    pub async fn vec_string_update_pop(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Option<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_pop, key)
        
    }

    pub async fn vec_string_update_append(&self, ctx: &Context<'_>, key: KeyType, value: Vec<String>) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_append, key, value)
        
    }

    pub async fn vec_string_update_clear(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_clear, key)
        
    }

    pub async fn vec_string_update_split_off(&self, ctx: &Context<'_>, key: KeyType, at: usize) -> async_graphql::Result<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_split_off, key, at)
        
    }

    pub async fn vec_string_update_resize(&self, ctx: &Context<'_>, key: KeyType, new_len: usize, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_resize, key, new_len, value)
        
    }

    pub async fn vec_string_update_dedup(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_dedup, key)
        
    }

    pub async fn vec_string_update_sort_unstable(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_sort_unstable, key)
        
    }

    pub async fn vec_string_update_rotate_left(&self, ctx: &Context<'_>, key: KeyType, mid: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_rotate_left, key, mid)
        
    }

    pub async fn vec_string_update_rotate_right(&self, ctx: &Context<'_>, key: KeyType, mid: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_rotate_right, key, mid)
        
    }

    pub async fn vec_string_update_fill(&self, ctx: &Context<'_>, key: KeyType, value: String) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_fill, key, value)
        
    }

    pub async fn vec_string_update_sort(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_sort, key)
        
    }

    pub async fn vec_string_update_swap(&self, ctx: &Context<'_>, key: KeyType, a: usize, b: usize) -> async_graphql::Result<&'static str>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_swap, key, a, b)
        
    }

    pub async fn vec_string_update_retrieve_contents(&self, ctx: &Context<'_>, key: KeyType) -> async_graphql::Result<Vec<String>>
    {

        call_store_method!(ctx, get_vec_string_namespace_ref, update_retrieve_contents, key)
        
    }

}

#[cfg(not(any(feature = "all_types", feature = "Vec_String")))]
#[Object]
impl VecStringMutation
{

    #[graphql(visible = false)]
    pub async fn vec_string_type_is_not_avalible(&self) -> Option<bool>
    {

        None

    }

}
