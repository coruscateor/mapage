//use crate::builds::levels::namespace_and_sub_namespace::collections::vec_collection_fns;

//constructors

#[macro_export]
macro_rules! impl_vec_new
{

    //$type_name:ident, 

    ($key_type:ty) =>
    {

        //[<$type_name _vec_new>]

        paste! {

            pub async fn vec_new(&self, key: &$key_type) -> async_graphql::Result<&'static str>
            {

                let my_vec = vec_new();

                self.namespace.insert(key, my_vec).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_vec_with_capacity
{

    ($key_type:ty) =>
    {

        paste! {

            pub async fn vec_with_capacity(&self, key: &$key_type, capacity: usize) -> async_graphql::Result<&'static str>
            {

                let my_vec = vec_with_capacity(capacity);

                self.namespace.insert(key, my_vec).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_vec_with_no_capacity
{

    ($key_type:ty) =>
    {

        paste! {

            pub async fn vec_with_no_capacity(&self, key: &$key_type) -> async_graphql::Result<&'static str>
            {

                let my_vec = vec_with_no_capacity();

                self.namespace.insert(key, my_vec).await

            }

        }

    }

}

//Queries

#[macro_export]
macro_rules! impl_vec_capacity
{

    ($key_type:ty) =>
    {

        paste! {

            pub async fn vec_capacity(&self, key: &$key_type, capacity: usize) -> async_graphql::Result<usize>
            {

                let my_fn = get_vec_capacity_fn(capacity);

                self.namespace.read_fn(key, my_fn).await

            }

        }

    }

}

#[macro_export]
macro_rules! impl_vec_len
{

    ($key_type:ty) =>
    {

        paste! {

            pub async fn vec_len(&self, key: &$key_type) -> async_graphql::Result<&'static str>
            {

                let my_fn = get_vec_len_fn();

                self.namespace.read_fn(key, my_fn).await

            }

        }

    }

}


