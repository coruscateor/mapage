use paste::paste;

//mutation

#[macro_export]
macro_rules! impl_store_model_body_insert
{

    ($ctx:ident, $label:ident, $key:ident, $value:ident) => //$key_type:ty, $value_type:ty) =>
    {

        //pub async fn [<$label _insert>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _insert>]($key, $value).await

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_model_body_update
{

    ($ctx:ident, $label:ident, $key:ident, $value:ident) =>   //$key_type:ty, $value_type:ty) =>
    {

        //pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _update>]($key, $value).await

        }

    }

}

//mutation

//update_fn

#[macro_export]
macro_rules! impl_store_model_update_fn
{

    //pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>

    ($ctx:ident, $label:ident, $key:ident) => //, $return:ident) =>
    {

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _update>]($key).await

        }

    }

}

//mutation

//update_fn - params

#[macro_export]
macro_rules! impl_store_model_update_fn_param
{

    ($ctx:ident, $label:ident, $key:ident, $param:ident) => //$field:ident, $updater_fn_name:ident, 
    {

        //single

        //pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _update>]($key, $param).await

        }

    };
    ($ctx:ident, $label:ident, $key:ident, $($parameter_name:ident)*) => //$($parameter_name:ident: $name_type:ty)* //$field:ident, $updater_fn_name:ident, 
    {

        //multi

        //pub async fn [<$label _update>](&self, ctx: &Context<'_>, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _update>]($key, $($parameter_name,)*).await

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_model_upsert
{

    ($ctx:ident, $label:ident, $key:ident, $value:ident) =>
    {

        //pub async fn [<$label _upsert>](&self, ctx: &Context<'_>, key: $key_type, value: $value_type) -> async_graphql::Result<UnitValue>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _upsert>]($key, $value).await

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_model_remove
{

    ($ctx:ident, $label:ident, $key:ident) =>
    {

        //pub async fn [<$label _remove>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<UnitValue>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _remove>]($key).await

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_model_read
{

    ($ctx:ident, $label:ident, $key:ident) =>
    {

        //pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _read>]($key).await

        }

    }

}

//query

//read_fn

#[macro_export]
macro_rules! impl_store_model_read_fn
{

    ($ctx:ident, $label:ident, $field:ident, $key_type:ty, $return_type:ty, $reader_fn_name:ident) =>
    {

        //pub async fn [<$label _read](&self, ctx: &Context<'_>, key: $key_type) -> async_graphql::Result<$return_type>

        paste! {

            
            {

                //self.$field.read_fn(&key, $reader_fn_name);

                $ctx.data_unchecked::<Store>().[<$label _read_fn>](key).await

            }

        }

    }

}

//query

//read_fn - params

#[macro_export]
macro_rules! impl_store_model_read_fn_param
{

    ($ctx:ident, $label:ident, $key:ident, $param:ident) => //, $return_type:ty
    {

        //single

        //pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type, param: $param_type) -> async_graphql::Result<$return_type>

        paste! {

            
            //self.$field.read_fn_param(&key, &param);

            $ctx.data_unchecked::<Store>().[<$label _read_fn_param>]($key, $param).await

        }

    };
    ($ctx:ident, $label:ident, $key:ident, $($parameter_name:ident: $name_type:ty)*) => //$field:ident, $reader_fn_name:ident,
    {

        //multi

        //pub async fn [<$label _read>](&self, ctx: &Context<'_>, key: $key_type, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>

        paste! {

            //self.$field.read_fn_param(&key, $($parameter_name,)*);

            $ctx.data_unchecked::<Store>().[<$label _read_fn_param>]($key, $($parameter_name,)*).await

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_model_contains
{

    ($ctx:ident, $label:ident, $key:ident) =>
    {

        //pub async fn [<$label _contains>](&self, ctx: &Context<'_>, key: &$key_type) -> bool

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _contains>]($key).await

        }

    }

}

//mutation

#[macro_export]
macro_rules! impl_store_model_clear
{

    ($ctx:ident, $label:ident) =>
    {

        //pub async fn [<$label _clear>](&self, ctx: &Context<'_>) -> usize

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _clear>]().await

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_model_len
{

    ($ctx:ident, $label:ident) =>
    {

        //pub async fn [<$label _len>](&self, ctx: &Context<'_>) -> usize

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _len>]().await

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_model_is_empty
{

    ($ctx:ident, $label:ident) =>
    {

        //pub async fn [<$label _is_empty>](&self, ctx: &Context<'_>) -> bool

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _is_empty>]().await

        }

    }

}

//query

#[macro_export]
macro_rules! impl_store_model_capacity
{

    ($ctx:ident, $label:ident) =>
    {

        //pub async fn [<$label _capacity>](&self, ctx: &Context<'_>) -> usize

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _capacity>]().await

        }

    }

}

//query

//get_all_keys

#[macro_export]
macro_rules! impl_store_model_get_all_keys
{

    ($ctx:ident, $label:ident, $key_type:ty) => //$return_type:ty
    {

        //pub async fn [<$label _get_all_keys>](&self, ctx: &Context<'_>) -> Vec<$key_type> //$return_type

        paste! {

            $ctx.data_unchecked::<Store>().[<$label _get_all_keys>]().await

        }

    }

}


