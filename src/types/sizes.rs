use std::mem::{size_of, size_of_val};

//#[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] 

//use super::{async_graphql_values::{Whatever, SelectedType}};

/*
#[cfg(any(feature = "all_types", feature = "Whatever"))]
use super::async_graphql_values::Whatever;

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
use super::async_graphql_values::SelectedType;
*/

//unit_value::*,

pub fn size_of_bool() -> usize
{

    size_of::<bool>()

}

pub fn size_of_char() -> usize
{

    size_of::<char>()

}

pub fn size_of_f32() -> usize
{

    size_of::<f32>()

}

pub fn size_of_f64() -> usize
{

    size_of::<f64>()

}

pub fn size_of_i8() -> usize
{

    size_of::<i8>()

}

pub fn size_of_i16() -> usize
{

    size_of::<i16>()

}

pub fn size_of_i32() -> usize
{

    size_of::<i32>()

}

pub fn size_of_i64() -> usize
{

    size_of::<i64>()

}

pub fn size_of_i128() -> usize
{

    size_of::<i128>()

}

pub fn size_of_isize() -> usize
{

    size_of::<isize>()

}

pub fn size_of_u8() -> usize
{

    size_of::<u8>()

}

pub fn size_of_u16() -> usize
{

    size_of::<u16>()

}

pub fn size_of_u32() -> usize
{

    size_of::<u32>()

}

pub fn size_of_u64() -> usize
{

    size_of::<u64>()

}

pub fn size_of_u128() -> usize
{

    size_of::<u128>()

}

/*
pub fn size_of_unit_value() -> usize
{

    size_of::<UnitValue>()

}

pub fn size_of_unit_value_combined() -> usize
{

    size_of::<UnitValue>() + size_of_unit_value_str()

}
*/

pub fn size_of_usize() -> usize
{

    size_of::<usize>()

}

pub fn size_of_string() -> usize
{

    size_of::<String>()

}

//to be used with values of kvps

pub fn size_of_string_combined(val: &String) -> usize
{

    size_of_string() + size_of_val(val)

}

pub fn size_of_len(val: &String) -> usize
{

    val.len() * size_of_char()

}

pub fn size_of_len_combined(val: &String) -> usize
{

    size_of_string() + size_of_len(val)

}

/*
#[cfg(any(feature = "all_types", feature = "Whatever"))]
pub fn size_of_whatever() -> usize
{

    size_of::<Whatever>()

}

#[cfg(any(feature = "all_types", feature = "SelectedType"))]
pub fn size_of_selected_type() -> usize
{

    size_of::<SelectedType>()

}
*/

#[macro_export]
macro_rules! impl_get_type_sizes
{

    () => {



    }

}
