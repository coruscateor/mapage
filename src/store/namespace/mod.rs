//! Namespace level astomicity and below.

pub mod scc_crate;

pub mod dashmap_crate;

#[cfg(any(feature = "all_types", feature = "bool"))]
pub mod bool_namespace;

#[cfg(any(feature = "all_types", feature = "char"))]
pub mod char_namespace;

#[cfg(any(feature = "all_types", feature = "string"))]
pub mod string_namespace;

#[cfg(any(feature = "all_types", feature = "whatever"))]
pub mod whatever_namespace;

//#[cfg(any(feature = "all_types", feature = "SelectedType"))] //, feature = "SelectedTypeIO"))]
//pub mod selected_type_namespace;

//feature = "isize", feature = "usize"

#[cfg(any(feature = "all_types", feature = "f32", feature = "f64", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128"))]
pub mod numeric_namespace;

pub mod collections;

mod store;

pub use store::*;




