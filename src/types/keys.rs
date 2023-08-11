
use paste::paste;

/*
#[cfg(any(feature = "all_key_types_String", feature = "bool_key_type_String"))]
type bool_key_type = String;

#[cfg(any(feature = "all_key_types_Arc_String", feature = "bool_key_type_Arc_String"))]
type bool_key_type = Arc<String>;
*/

macro_rules! key_type_features
{

    ($for_type:ident) =>
    {

        //[<$for_type _>]

        paste! {

            #[cfg(any(feature = "all_key_types_String", feature = "[<$for_type _key_type_String>]"))]
            pub type [<$for_type _key_type>] = String;

            #[cfg(any(feature = "all_key_types_Arc_String", feature = "[<$for_type _key_type_Arc_String>]"))]
            pub type [<$for_type _key_type>] = Arc<String>;

        }

    } 

}

#[cfg(any(feature = "all_types", feature = "bool"))]
key_type_features!(bool);

#[cfg(any(feature = "all_types", feature = "char"))]
key_type_features!(char);

#[cfg(any(feature = "all_types", feature = "f32"))]
key_type_features!(f32);

#[cfg(any(feature = "all_types", feature = "f64"))]
key_type_features!(f64);

#[cfg(any(feature = "all_types", feature = "i8"))]
key_type_features!(i8);

#[cfg(any(feature = "all_types", feature = "i16"))]
key_type_features!(i16);

#[cfg(any(feature = "all_types", feature = "i32"))]
key_type_features!(i32);

#[cfg(any(feature = "all_types", feature = "i64"))]
key_type_features!(i64);

#[cfg(any(feature = "all_types", feature = "i128"))]
key_type_features!(i128);

#[cfg(any(feature = "all_types", feature = "isize"))]
key_type_features!(isize);

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
key_type_features!(SelectedType);

#[cfg(any(feature = "all_types", feature = "String"))]
key_type_features!(String);

#[cfg(any(feature = "all_types", feature = "u8"))]
key_type_features!(u8);

#[cfg(any(feature = "all_types", feature = "u16"))]
key_type_features!(u16);

#[cfg(any(feature = "all_types", feature = "u32"))]
key_type_features!(u32);

#[cfg(any(feature = "all_types", feature = "u64"))]
key_type_features!(u64);

#[cfg(any(feature = "all_types", feature = "u128"))]
key_type_features!(u128);

#[cfg(any(feature = "all_types", feature = "usize"))]
key_type_features!(usize);

#[cfg(any(feature = "all_types", feature = "Whatever"))]
key_type_features!(Whatever);

