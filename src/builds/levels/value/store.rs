use scc::HashMap;
//use super::mutex::{mutex_bool_value::MutexBoolValue, mutex_char_value::MutexCharValue, mutex_numeric_value::MutexNumericValue, mutex_string_value::MutexStringValue};

use corlib::has_one::*;

use crate::errors::invalid_operation;

use crate::types::UnitValue;

use paste::paste;

use crate::impl_scc_hashmap_all_methods;

pub struct Store
{

    //scalar values 

    bool_values: HashMap<String, bool>,
    char_values: HashMap<String, char>,
    f32_values: HashMap<String, f32>, //MutexNumericValue<f32, F32HasOne>>,
    f64_values: HashMap<String, f64>,
    i8_values: HashMap<String, i8>,
    i16_values: HashMap<String, i16>,
    i32_values: HashMap<String, i32>,
    i64_values: HashMap<String, i64>,
    i128_values: HashMap<String, i128>,
    isize_values: HashMap<String, isize>,
    u8_values: HashMap<String, u8>,
    u16_values: HashMap<String, u16>,
    u32_values: HashMap<String, u32>,
    u64_values: HashMap<String, u64>,
    u128_values: HashMap<String, u128>,
    unit_values: HashMap<String, UnitValue>,
    usize_values: HashMap<String, usize>,

    //vector values
    
    string_values: HashMap<String, String>,

}

impl Store
{

    impl_scc_hashmap_all_methods!(bool, bool_values, bool);

    impl_scc_hashmap_all_methods!(char, char_values, char);

    impl_scc_hashmap_all_methods!(f32, f32_values, f32);

    impl_scc_hashmap_all_methods!(f64, f64_values, f64);

    impl_scc_hashmap_all_methods!(i8, i8_values, i8);

    impl_scc_hashmap_all_methods!(i16, i16_values, i16);

    impl_scc_hashmap_all_methods!(i32, i32_values, i32);

    impl_scc_hashmap_all_methods!(i64, i64_values, i64);

    impl_scc_hashmap_all_methods!(i128, i128_values, i128);

    impl_scc_hashmap_all_methods!(isize, isize_values, isize);

    impl_scc_hashmap_all_methods!(u8, u8_values, u8);

    impl_scc_hashmap_all_methods!(u16, u16_values, u16);

    impl_scc_hashmap_all_methods!(u32, u32_values, u32);

    impl_scc_hashmap_all_methods!(u64, u64_values, u64);

    impl_scc_hashmap_all_methods!(u128, u128_values, u128);

    impl_scc_hashmap_all_methods!(unit_value, unit_values, UnitValue);

    impl_scc_hashmap_all_methods!(usize, usize_values, usize);

    impl_scc_hashmap_all_methods!(string, string_values, String);

}


