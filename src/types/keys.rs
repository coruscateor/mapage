
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
            pub type [<$for_type KeyType>] = String; //[<$for_type _key_type>]

            #[cfg(any(feature = "all_key_types_Arc_String", feature = "[<$for_type _key_type_Arc_String>]"))]
            pub type [<$for_type KeyType>] = Arc<String>; //[<$for_type _key_type>]

        }

    } 

}

#[cfg(any(feature = "all_types", feature = "bool"))]
key_type_features!(Bool);

#[cfg(any(feature = "all_types", feature = "char"))]
key_type_features!(Char);

#[cfg(any(feature = "all_types", feature = "f32"))]
key_type_features!(F32);

#[cfg(any(feature = "all_types", feature = "f64"))]
key_type_features!(F64);

#[cfg(any(feature = "all_types", feature = "i8"))]
key_type_features!(I8);

#[cfg(any(feature = "all_types", feature = "i16"))]
key_type_features!(I16);

#[cfg(any(feature = "all_types", feature = "i32"))]
key_type_features!(I32);

#[cfg(any(feature = "all_types", feature = "i64"))]
key_type_features!(I64);

#[cfg(any(feature = "all_types", feature = "i128"))]
key_type_features!(I128);

#[cfg(any(feature = "all_types", feature = "isize"))]
key_type_features!(Isize);

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
key_type_features!(SelectedType);

#[cfg(any(feature = "all_types", feature = "String"))]
key_type_features!(String);

#[cfg(any(feature = "all_types", feature = "u8"))]
key_type_features!(U8);

#[cfg(any(feature = "all_types", feature = "u16"))]
key_type_features!(U16);

#[cfg(any(feature = "all_types", feature = "u32"))]
key_type_features!(U32);

#[cfg(any(feature = "all_types", feature = "u64"))]
key_type_features!(U64);

#[cfg(any(feature = "all_types", feature = "u128"))]
key_type_features!(U128);

#[cfg(any(feature = "all_types", feature = "usize"))]
key_type_features!(Usize);

#[cfg(any(feature = "all_types", feature = "Whatever"))]
key_type_features!(Whatever);

