//mod vec_namespace;

#[cfg(any(feature = "all_types", feature = "Vec_bool"))]
pub mod vec_bool_namespace;

pub mod vec_collection_fns;

pub mod vec_collection_macros;

#[cfg(any(feature = "all_types", feature = "Vec_char"))]
pub mod vec_char_namespace;

pub mod vec_numeric_namespace;

//#[cfg(any(feature = "all_types", feature = "VecSelectedType"))]
//pub mod vec_selected_type_namespace;

#[cfg(any(feature = "all_types", feature = "VecString"))]
pub mod vec_string_namespace;

//#[cfg(any(feature = "all_types", feature = "VecWhatever"))]
//pub mod vec_whatever_namespace;




