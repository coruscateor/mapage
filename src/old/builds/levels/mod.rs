
#[cfg(any(feature = "store_aml", feature = "sub_store_aml"))]
pub mod namespace_and_sub_namespace;

#[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
pub mod none;

