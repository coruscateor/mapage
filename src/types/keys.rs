
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

        //Support varied key-types per supported type?

        paste! {

            #[cfg(any(feature = "string_keys"))] //, feature = "[<$for_type _key_type_String>]"))]
            pub type [<$for_type KeyType>] = String; //[<$for_type _key_type>]

            #[cfg(any(feature = "arc_str_keys"))] //, feature = "[<$for_type _key_type_Arc_String>]"))]
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

/*
#[cfg(any(feature = "all_types", feature = "isize"))]
key_type_features!(ISize);

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
key_type_features!(SelectedType);
*/

#[cfg(any(feature = "all_types", feature = "string"))]
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

/*
#[cfg(any(feature = "all_types", feature = "usize"))]
key_type_features!(USize);
*/

#[cfg(any(feature = "all_types", feature = "whatever"))]
key_type_features!(Whatever);

//Collections

#[cfg(any(feature = "all_types", feature = "vec_bool"))]
key_type_features!(VecBool);

/*
#[cfg(any(feature = "all_types", feature = "vec_char"))]
key_type_features!(VecChar);
*/

#[cfg(any(feature = "all_types", feature = "vec_f32"))]
key_type_features!(VecF32);

#[cfg(any(feature = "all_types", feature = "vec_f64"))]
key_type_features!(VecF64);

#[cfg(any(feature = "all_types", feature = "vec_i8"))]
key_type_features!(VecI8);

#[cfg(any(feature = "all_types", feature = "vec_i16"))]
key_type_features!(VecI16);

#[cfg(any(feature = "all_types", feature = "vec_i32"))]
key_type_features!(VecI32);

#[cfg(any(feature = "all_types", feature = "vec_i64"))]
key_type_features!(VecI64);

#[cfg(any(feature = "all_types", feature = "vec_i128"))]
key_type_features!(VecI128);

/*
#[cfg(any(feature = "all_types", feature = "vec_isize"))]
key_type_features!(VecISize);

#[cfg(any(feature = "all_types", feature = "Vec_SelectedType"))]
key_type_features!(VecSelectedType);

#[cfg(any(feature = "all_types", feature = "vec_string"))]
key_type_features!(VecString);
*/

#[cfg(any(feature = "all_types", feature = "vec_u8"))]
key_type_features!(VecU8);

#[cfg(any(feature = "all_types", feature = "vec_u16"))]
key_type_features!(VecU16);

#[cfg(any(feature = "all_types", feature = "vec_u32"))]
key_type_features!(VecU32);

#[cfg(any(feature = "all_types", feature = "vec_u64"))]
key_type_features!(VecU64);

#[cfg(any(feature = "all_types", feature = "vec_u128"))]
key_type_features!(VecU128);

/*
#[cfg(any(feature = "all_types", feature = "vec_usize"))]
key_type_features!(VecUSize);
*/

#[cfg(any(feature = "all_types", feature = "vec_whatever"))]
key_type_features!(VecWhatever);

