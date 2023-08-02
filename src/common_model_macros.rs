use paste::paste;

//mutation

#[macro_export]
macro_rules! impl_store_container_insert
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _insert>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                ctx.data_unchecked::<Store>().[<$label _insert>](key, value).await

            }

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_container_update
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                ctx.data_unchecked::<Store>().[<$label _update>](key, value).await

            }

        }

    }

}

//mutation

//update_fn

#[macro_export]
macro_rules! impl_store_container_update_fn
{

    ($label:ident, $key_type:ty, $return_type:ty) => //$field:ident, $updater_fn_name:ident,
    {

        paste! {

            pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>
            {

                //self.$field.update_fn(key, $updater_fn_name).await

                ctx.data_unchecked::<Store>().[<$label _update>](key).await

            }

        }

    }

}

//mutation

//update_fn - params

#[macro_export]
macro_rules! impl_store_container_update_fn_param
{

    ($label:ident, $key_type:ty, $param_type:ty, $return_type:ty) => //$field:ident, $updater_fn_name:ident, 
    {

        //single

        paste! {

            pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>
            {

                //self.$field.update_fn_param(key, $updater_fn_name, param).await

                ctx.data_unchecked::<Store>().[<$label _update>](key, param).await

            }

        }

    };
    ($label:ident, $key_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) => //$field:ident, $updater_fn_name:ident, 
    {

        //multi

        paste! {

            pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type> //$param_type, //, updater: F //_fn //<UnitValue>
            {

                //let param = ($($parameter_name,)*);

                ctx.data_unchecked::<Store>().[<$label _update>](key, ($parameter_name,)*).await

            }

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_container_upsert
{

    ($label:ident, $key_type:ty, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                ctx.data_unchecked::<Store>().[<$label _upsert>](key, value).await

            }

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_container_remove
{

    ($label:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _remove>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<UnitValue>
            {

                ctx.data_unchecked::<Store>().[<$label _remove>](key).await

            }

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_container_read
{

    ($label:ident, $key_type:ty, $return_type:ty) => //$key_type:ty, 
    {

        paste! {

            pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>
            {

                ctx.data_unchecked::<Store>().[<$label _read>](key).await

            }

        }

    }

}

//query

//read_fn

#[macro_export]
macro_rules! impl_store_container_read_fn
{

    ($label:ident, $field:ident, $key_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        paste! {

            pub async fn [<$label _read](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>
            {

                //self.$field.read_fn(&key, $reader_fn_name);

                ctx.data_unchecked::<Store>().[<$label _read>](key).await

            }

        }

    }

}

//query

//read_fn - params

#[macro_export]
macro_rules! impl_store_container_read_fn_param
{

    ($label:ident, $key_type:ty, $param_type:ty, $return_type:ty) => //$field:ident, , $reader_fn_name:ident
    {

        //single

        paste! {

            pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>
            {

                //let reader = $get_reader_fn();

                self.$field.read_fn_param(&key, &param); //$reader_fn_name,

            }

        }

    };
    ($label:ident, $key_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) => //$field:ident, $reader_fn_name:ident,
    {

        //multi

        paste! {

            pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                //let param = ($($parameter_name,)*);

                self.$field.read_fn_param(&key, $($parameter_name,)*);

            }

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_container_contains
{

    ($label:ident, $key_type:ty) =>
    {

        paste! {

            pub async fn [<$label _contains>](&self, ctx: &Context<'_>, key: &$key_type) -> bool
            {

                ctx.data_unchecked::<Store>().[<$label _contains>](key).await

            }

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_container_clear
{

    ($label:ident) =>
    {

        paste! {

            pub async fn [<$label _clear>](&self, ctx: &Context<'_>) -> usize
            {

                ctx.data_unchecked::<Store>().[<$label _clear>]().await

            }

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_container_len
{

    ($label:ident) =>
    {

        paste! {

            pub async fn [<$label _len>](&self, ctx: &Context<'_>) -> usize
            {

                ctx.data_unchecked::<Store>().[<$label _len>]().await

            }

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_container_is_empty
{

    ($label:ident) =>
    {

        paste! {

            pub async fn [<$label _is_empty>](&self, ctx: &Context<'_>) -> bool
            {

                ctx.data_unchecked::<Store>().[<$label _is_empty>]().await

            }

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_container_capacity
{

    ($label:ident) =>
    {

        paste! {

            pub async fn [<$label _capacity>](&self, ctx: &Context<'_>) -> usize
            {

                ctx.data_unchecked::<Store>().[<$label _capacity>]().await

            }

        }

    }

}

//query

//get_all_keys

#[macro_export]
macro_rules! impl_store_container_get_all_keys
{

    ($label:ident, $key_type:ty) => //$return_type:ty
    {

        paste! {

            pub async fn [<$label _get_all_keys>](&self, ctx: &Context<'_>) -> Vec<$key_type> //$return_type
            {

                ctx.data_unchecked::<Store>().[<$label _get_all_keys>]().await

            }

        }

    }

}


