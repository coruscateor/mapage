

#[macro_export]
macro_rules! call_store_method
{

    /*
    ($namespace_ref_method:ident, $method:ident, $(parameter:expr),*) =>
    {

        ctx.data_unchecked::<StoreType>().$namespace_ref_method.$method($(parameter),*).await

    };*/
    ($ctx:ident, $namespace_ref_method:ident, $method:ident, $key:expr $(,$parameter:expr)*) =>
    {

        $ctx.data_unchecked::<StoreType>().$namespace_ref_method().$method(&$key $(,$parameter)*).await

    }

}

#[macro_export]
macro_rules! call_store_method_only_move_key
{

    ($ctx:ident, $namespace_ref_method:ident, $method:ident, $key:expr $(,$parameter:expr)*) =>
    {

        $ctx.data_unchecked::<StoreType>().$namespace_ref_method().$method($key $(,$parameter)*).await

    }

}

#[macro_export]
macro_rules! call_store_method_no_key
{

    ($ctx:ident, $namespace_ref_method:ident, $method:ident $(,$parameter:expr)*) =>
    {

        $ctx.data_unchecked::<StoreType>().$namespace_ref_method().$method($($parameter),*).await

    }
    
}
