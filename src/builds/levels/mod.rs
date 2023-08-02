//pub mod store;

//pub mod namespace;

//pub mod value;

#[cfg(any(feature = "store_aml", feature = "sub_store_aml"))]
pub mod namespace_and_sub_namespace;

#[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
pub mod none;

/*

Managed Namespace
|
Type Specific Namespace
|
General Namespace

 */