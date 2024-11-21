
//Imports

#[macro_export]
macro_rules! impl_vec_collection_fns_imports
{

    () =>
    {

        use crate::store::namespace::collections::vec_collection_fns::*;

        //use crate::builds::levels::namespace_and_sub_namespace::collections::vec_collection_fns::*;

    }

}

//Constructors

#[macro_export]
macro_rules! impl_vec_insert_new
{

    //$type_name:ident, 

    ($key_type:ty) =>
    {

        //[<$type_name _vec_new>]

        pub async fn insert_new(&self, key: $key_type) -> Result<&'static str>
        {

            let my_vec = vec_new();

            self.namespace.insert(key, my_vec).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_insert_with_capacity
{

    ($key_type:ty) =>
    {

        pub async fn insert_with_capacity(&self, key: $key_type, capacity: usize) -> Result<&'static str>
        {

            let my_vec = vec_with_capacity(capacity);

            self.namespace.insert(key, my_vec).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_insert_with_no_capacity
{

    ($key_type:ty) =>
    {

        pub async fn insert_with_no_capacity(&self, key: $key_type) -> Result<&'static str>
        {

            let my_vec = vec_with_no_capacity();

            self.namespace.insert(key, my_vec).await

        }

    }

}

//Queries

#[macro_export]
macro_rules! impl_vec_read_capacity
{

    ($key_type:ty) =>
    {

        pub async fn read_capacity(&self, key: &$key_type) -> Result<usize>
        {

            let my_fn = get_vec_capacity_fn();

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_read_len
{

    ($key_type:ty) =>
    {

        pub async fn read_len(&self, key: &$key_type) -> Result<usize>
        {

            let my_fn = get_vec_len_fn();

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_read_is_empty
{

    ($key_type:ty) =>
    {

        pub async fn read_is_empty(&self, key: &$key_type) -> Result<bool>
        {

            let my_fn = get_vec_is_empty_fn();

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_read_first
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn read_first(&self, key: &$key_type) -> Result<Option<$value_type>>
        {

            let my_fn = get_vec_first_fn();

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_read_last
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn read_last(&self, key: &$key_type) -> Result<Option<$value_type>>
        {

            let my_fn = get_vec_last_fn();

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

//Requres Ord

#[macro_export]
macro_rules! impl_vec_read_binary_search
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn read_binary_search(&self, key: &$key_type, x: $value_type) -> Result<usize>
        {

            let my_fn = get_vec_binary_search_fn(x);

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

#[macro_export]
macro_rules! impl_vec_read_index
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn read_index(&self, key: &$key_type, index: usize) -> Result<$value_type>
        {

            let my_fn = get_vec_index_fn::<$value_type>(index);

            self.namespace.read_fn(key, my_fn).await

        }

    }

}

//Mutations

//index_mut

#[macro_export]
macro_rules! impl_vec_update_index_mut
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_index_mut(&self, key: &$key_type, index: usize, value: $value_type) -> Result<&'static str>
        {

            let my_fn = get_vec_index_mut_fn::<$value_type>(index, value);

            self.namespace.update_fn(key, my_fn).await

        }

    }

}

//reserve

#[macro_export]
macro_rules! impl_vec_update_reserve
{

    ($key_type:ty) =>
    {

        pub async fn update_reserve(&self, key: &$key_type, additional: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_reserve_fn(additional);

            self.namespace.update_fn(key, my_fn).await

        }

    }

}

//reserve_exact

#[macro_export]
macro_rules! impl_vec_update_reserve_exact
{

    ($key_type:ty) =>
    {

        pub async fn update_reserve_exact(&self, key: &$key_type, additional: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_reserve_exact_fn(additional);

            self.namespace.update_fn(key, my_fn).await

        }

    }

}

//try_reserve

#[macro_export]
macro_rules! impl_vec_update_try_reserve
{

    ($key_type:ty) =>
    {

        pub async fn update_try_reserve(&self, key: &$key_type, additional: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_try_reserve_fn(additional);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//try_reserve_exact

#[macro_export]
macro_rules! impl_vec_update_try_reserve_exact
{

    ($key_type:ty) =>
    {

        pub async fn update_try_reserve_exact(&self, key: &$key_type, additional: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_try_reserve_exact_fn(additional);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//shrink_to_fit

#[macro_export]
macro_rules! impl_vec_update_shrink_to_fit
{

    ($key_type:ty) =>
    {

        pub async fn update_shrink_to_fit(&self, key: &$key_type) -> Result<&'static str>
        {

            let my_fn = get_vec_shrink_to_fit_fn();

            self.namespace.update_fn(key, my_fn).await

        }

    }

}

//shrink_to

#[macro_export]
macro_rules! impl_vec_update_shrink_to
{

    ($key_type:ty) =>
    {

        pub async fn update_shrink_to(&self, key: &$key_type, min_capacity: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_shrink_to_fn(min_capacity);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//truncate

#[macro_export]
macro_rules! impl_vec_update_truncate
{

    ($key_type:ty) =>
    {

        pub async fn update_truncate(&self, key: &$key_type, len: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_truncate_fn(len);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//insert

#[macro_export]
macro_rules! impl_vec_update_insert
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_insert(&self, key: &$key_type, index: usize, element: $value_type) -> Result<&'static str>
        {

            let my_fn = get_vec_insert_fn(index, element);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//impl_vec_insert_method - with channel

//push

#[macro_export]
macro_rules! impl_vec_update_push
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_push(&self, key: &$key_type, value: $value_type) -> Result<&'static str>
        {

            let my_fn = get_vec_push_fn(value);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//impl_vec_push_method - with channel

//pop

#[macro_export]
macro_rules! impl_vec_update_pop
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_pop(&self, key: &$key_type) -> Result<Option<$value_type>>
        {

            let my_fn = get_vec_pop_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//append

/*
#[macro_export]
macro_rules! impl_vec_append_method
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn append_method(&self, key: &$key_type, value: &'static mut Vec<$value_type>) -> Result<&'static str>
        {

            let my_fn = get_vec_append_fn(value); //<'static, $value_type>

            self.namespace.update_fn(key, my_fn).await
            
        }
    }

}
*/

/*
#[macro_export]
macro_rules! impl_vec_append_method
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn append_method(&self, key: &$key_type, value: Vec<$value_type>) -> Result<&'static str>
        {

            let my_fn = get_vec_append_fn(&mut value);

            self.namespace.update_fn(key, my_fn).await
            
        }
    }

}
*/

#[macro_export]
macro_rules! impl_vec_update_append
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_append(&self, key: &$key_type, value: Vec<$value_type>) -> Result<&'static str>
        {

            let my_fn = get_vec_append_fn(value);

            self.namespace.update_fn(key, my_fn).await //.0
            
        }

    }

}

//impl_vec_append_method - with channel

/*
#[macro_export]
macro_rules! impl_vec_append_method_ref
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn append_method(&self, key: &$key_type, value: MoveOnly<&mut Vec<$value_type>>) -> Result<&'static str>
        {

            let my_fn = get_vec_append_ref_fn(value);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}
*/

//clear

#[macro_export]
macro_rules! impl_vec_update_clear
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_clear(&self, key: &$key_type) -> Result<&'static str>
        {

            let my_fn = get_vec_clear_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//split_off

#[macro_export]
macro_rules! impl_vec_update_split_off
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_split_off(&self, key: &$key_type, at: usize) -> Result<Vec<$value_type>>
        {

            let my_fn = get_vec_split_off_fn::<$value_type>(at);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//resize

#[macro_export]
macro_rules! impl_vec_update_resize
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_resize(&self, key: &$key_type, new_len: usize, value: $value_type) -> Result<&'static str>
        {

            let my_fn = get_vec_resize_fn(new_len, value);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//impl_vec_resize_method - With Channel

//dedup

//Requires PartialEq

#[macro_export]
macro_rules! impl_vec_update_dedup
{

    ($key_type:ty) =>
    {

        pub async fn update_dedup(&self, key: &$key_type) -> Result<&'static str>
        {

            let my_fn = get_vec_dedup_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//sort_unstable

//Requres Ord

#[macro_export]
macro_rules! impl_vec_update_sort_unstable
{

    ($key_type:ty) =>
    {

        pub async fn update_sort_unstable(&self, key: &$key_type) -> Result<&'static str>
        {

            let my_fn = get_vec_sort_unstable_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//rotate_left

//Requres Ord

#[macro_export]
macro_rules! impl_vec_update_rotate_left
{

    ($key_type:ty) =>
    {

        pub async fn update_rotate_left(&self, key: &$key_type, mid: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_rotate_left_fn(mid);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//rotate_right

//Requres Ord

#[macro_export]
macro_rules! impl_vec_update_rotate_right
{

    ($key_type:ty) =>
    {

        pub async fn update_rotate_right(&self, key: &$key_type, mid: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_rotate_right_fn(mid);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//fill

#[macro_export]
macro_rules! impl_vec_update_fill
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_fill(&self, key: &$key_type, value: $value_type) -> Result<&'static str>
        {

            let my_fn = get_vec_fill_fn(value);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//sort

//Requres Ord

#[macro_export]
macro_rules! impl_vec_update_sort
{

    ($key_type:ty) =>
    {

        pub async fn update_sort(&self, key: &$key_type) -> Result<&'static str>
        {

            let my_fn = get_vec_sort_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//swap

//Requres Ord

#[macro_export]
macro_rules! impl_vec_update_swap
{

    ($key_type:ty) =>
    {

        pub async fn update_swap(&self, key: &$key_type, a: usize, b: usize) -> Result<&'static str>
        {

            let my_fn = get_vec_swap_fn(a, b);

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//retrieve_contents_fn

#[macro_export]
macro_rules! impl_vec_update_retrieve_contents
{

    ($key_type:ty, $value_type:ty) =>
    {

        pub async fn update_retrieve_contents(&self, key: &$key_type) -> Result<Vec<$value_type>>
        {

            let my_fn = get_vec_retrieve_contents_fn();

            self.namespace.update_fn(key, my_fn).await
            
        }

    }

}

//

#[macro_export]
macro_rules! impl_vec_fns
{

    ($key_type:ty, $value_type:ty) =>
    {

        //Constructors

        crate::impl_vec_insert_new!($key_type);

        crate::impl_vec_insert_with_capacity!($key_type);

        crate::impl_vec_insert_with_no_capacity!($key_type);

        //Queries

        crate::impl_vec_read_capacity!($key_type);

        crate::impl_vec_read_len!($key_type);

        crate::impl_vec_read_is_empty!($key_type);

        crate::impl_vec_read_first!($key_type, $value_type);

        crate::impl_vec_read_last!($key_type, $value_type);

        crate::impl_vec_read_binary_search!($key_type, $value_type);

        crate::impl_vec_read_index!($key_type, $value_type);

        //Mutations

        crate::impl_vec_update_index_mut!($key_type, $value_type);

        crate::impl_vec_update_reserve!($key_type);

        crate::impl_vec_update_reserve_exact!($key_type);

        crate::impl_vec_update_try_reserve!($key_type);

        crate::impl_vec_update_try_reserve_exact!($key_type);

        crate::impl_vec_update_shrink_to_fit!($key_type);

        crate::impl_vec_update_shrink_to!($key_type);

        crate::impl_vec_update_truncate!($key_type);

        crate::impl_vec_update_insert!($key_type, $value_type);

        crate::impl_vec_update_push!($key_type, $value_type);

        crate::impl_vec_update_pop!($key_type, $value_type);

        crate::impl_vec_update_append!($key_type, $value_type);

        crate::impl_vec_update_clear!($key_type, $value_type);

        crate::impl_vec_update_split_off!($key_type, $value_type);

        crate::impl_vec_update_resize!($key_type, $value_type);

        crate::impl_vec_update_dedup!($key_type);

        crate::impl_vec_update_sort_unstable!($key_type);

        crate::impl_vec_update_rotate_left!($key_type);

        crate::impl_vec_update_rotate_right!($key_type);

        crate::impl_vec_update_fill!($key_type, $value_type);

        crate::impl_vec_update_sort!($key_type);

        crate::impl_vec_update_swap!($key_type);

        crate::impl_vec_update_retrieve_contents!($key_type, $value_type);

    }

}


#[macro_export]
macro_rules! impl_vec_fns_no_ord
{

    ($key_type:ty, $value_type:ty) =>
    {

        //Constructors

        crate::impl_vec_insert_new!($key_type);

        crate::impl_vec_insert_with_capacity!($key_type);

        crate::impl_vec_insert_with_no_capacity!($key_type);

        //Queries

        crate::impl_vec_read_capacity!($key_type);

        crate::impl_vec_read_len!($key_type);

        crate::impl_vec_read_is_empty!($key_type);

        crate::impl_vec_read_first!($key_type, $value_type);

        crate::impl_vec_read_last!($key_type, $value_type);

        //crate::impl_vec_binary_search_method!($key_type, $value_type);

        crate::impl_vec_read_index!($key_type, $value_type);

        //Mutations

        crate::impl_vec_update_index_mut!($key_type, $value_type);

        crate::impl_vec_update_reserve!($key_type);

        crate::impl_vec_update_reserve_exact!($key_type);

        crate::impl_vec_update_try_reserve!($key_type);

        crate::impl_vec_update_try_reserve_exact!($key_type);

        crate::impl_vec_update_shrink_to_fit!($key_type);

        crate::impl_vec_update_shrink_to!($key_type);

        crate::impl_vec_update_truncate!($key_type);

        crate::impl_vec_update_insert!($key_type, $value_type);

        crate::impl_vec_update_push!($key_type, $value_type);

        crate::impl_vec_update_pop!($key_type, $value_type);

        crate::impl_vec_update_append!($key_type, $value_type);

        crate::impl_vec_update_clear!($key_type, $value_type);

        crate::impl_vec_update_split_off!($key_type, $value_type);

        crate::impl_vec_update_resize!($key_type, $value_type);

        crate::impl_vec_update_dedup!($key_type);

        //crate::impl_vec_sort_unstable_method!($key_type);

        //crate::impl_vec_rotate_left_method!($key_type);

        //crate::impl_vec_rotate_right_method!($key_type);

        //crate::impl_vec_fill_method!($key_type, $value_type);

        //crate::impl_vec_sort_method!($key_type);

        //crate::impl_vec_swap_method!($key_type);
        
        crate::impl_vec_update_retrieve_contents!($key_type, $value_type);

    }

}

#[macro_export]
macro_rules! impl_vec_fns_ord_only
{

    ($key_type:ty, $value_type:ty) =>
    {

        //Queries

        crate::impl_vec_read_binary_search!($key_type, $value_type);

        //Mutations

        //crate::impl_vec_update_index_mut!($key_type, $value_type); //duplicate definitions with name `update_index_mut`
        
        crate::impl_vec_update_sort_unstable!($key_type);
       
        crate::impl_vec_update_rotate_left!($key_type);
       
        crate::impl_vec_update_rotate_right!($key_type);
       
        crate::impl_vec_update_fill!($key_type, $value_type);
       
        crate::impl_vec_update_sort!($key_type);
       
        crate::impl_vec_update_swap!($key_type);

    }

}

#[macro_export]
macro_rules! impl_vec_fns_no_ord_or_partial_eq
{

    ($key_type:ty, $value_type:ty) =>
    {

        //Constructors

        crate::impl_vec_insert_new!($key_type);

        crate::impl_vec_insert_with_capacity!($key_type);

        crate::impl_vec_insert_with_no_capacity!($key_type);

        //Queries

        crate::impl_vec_read_capacity!($key_type);

        crate::impl_vec_read_len!($key_type);

        crate::impl_vec_read_is_empty!($key_type);

        crate::impl_vec_read_first!($key_type, $value_type);

        crate::impl_vec_read_last!($key_type, $value_type);

        //crate::impl_vec_binary_search_method!($key_type, $value_type);

        crate::impl_vec_read_index!($key_type, $value_type);

        //Mutations

        crate::impl_vec_update_index_mut!($key_type, $value_type);

        crate::impl_vec_update_reserve!($key_type);

        crate::impl_vec_update_reserve_exact!($key_type);

        crate::impl_vec_update_try_reserve!($key_type);

        crate::impl_vec_update_try_reserve_exact!($key_type);

        crate::impl_vec_update_shrink_to_fit!($key_type);

        crate::impl_vec_update_shrink_to!($key_type);

        crate::impl_vec_update_truncate!($key_type);

        crate::impl_vec_update_insert!($key_type, $value_type);

        crate::impl_vec_update_push!($key_type, $value_type);

        crate::impl_vec_update_pop!($key_type, $value_type);

        crate::impl_vec_update_append!($key_type, $value_type);

        crate::impl_vec_update_clear!($key_type, $value_type);

        crate::impl_vec_update_split_off!($key_type, $value_type);

        crate::impl_vec_update_resize!($key_type, $value_type);

        //crate::impl_vec_update_dedup!($key_type);

        //crate::impl_vec_dedup_method!($key_type);

        //crate::impl_vec_sort_unstable_method!($key_type);

        //crate::impl_vec_rotate_left_method!($key_type);

        //crate::impl_vec_rotate_right_method!($key_type);

        //crate::impl_vec_fill_method!($key_type, $value_type);

        //crate::impl_vec_sort_method!($key_type);

        //crate::impl_vec_swap_method!($key_type);
        
        crate::impl_vec_update_retrieve_contents!($key_type, $value_type);

    }

}
