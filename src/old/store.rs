//Which store to use?

cfg_if::cfg_if! 
{

    if #[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
    {

        //use crate::builds::levels::none::store::Store;
    
        pub type StoreType = crate::builds::levels::none::store::Store;

    }
    else if #[cfg(feature = "sub_store_aml")]
    {

        
        //use crate::builds::levels::namespace_and_sub_namespace::store::Store;

        pub type Store = crate::builds::levels::namespace_and_sub_namespace::store::Store;

    }

}


