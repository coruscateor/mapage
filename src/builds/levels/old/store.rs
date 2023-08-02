use scc::HashMap;
//use super::mutex::{mutex_bool_value::MutexBoolValue, mutex_char_value::MutexCharValue, mutex_numeric_value::MutexNumericValue, mutex_string_value::MutexStringValue};

use corlib::has_one::*;

use crate::errors::invalid_operation;

use crate::types::UnitValue;
use crate::types::async_graphql_values::{I128Scalar, U128Scalar};

use paste::paste;

use crate::{impl_scc_hashmap_all_methods_copy, impl_scc_hashmap_all_methods_clone, impl_scc_hashmap_update_function_call, impl_scc_hashmap_upsert_function_call, impl_scc_hashmap_update_upsert_function_calls};

use crate::types::scalar_type_functions::{bool_bit_and, bool_bit_or, bool_bit_xor, bool_not};

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
    //i128_values: HashMap<String, i128>,
    i128_values: HashMap<String, I128Scalar>,
    isize_values: HashMap<String, isize>,
    u8_values: HashMap<String, u8>,
    u16_values: HashMap<String, u16>,
    u32_values: HashMap<String, u32>,
    u64_values: HashMap<String, u64>,
    //u128_values: HashMap<String, u128>,
    u128_values: HashMap<String, U128Scalar>,
    unit_values: HashMap<String, UnitValue>,
    usize_values: HashMap<String, usize>,

    //vector values
    
    string_values: HashMap<String, String>,

}

impl Store
{

    pub fn new() -> Self
    {

        Self
        {

            bool_values: HashMap::new(),
            char_values: HashMap::new(),
            f32_values: HashMap::new(),
            f64_values: HashMap::new(),
            i8_values: HashMap::new(),
            i16_values: HashMap::new(),
            i32_values: HashMap::new(),
            i64_values: HashMap::new(),
            i128_values: HashMap::new(),
            isize_values: HashMap::new(),
            u8_values: HashMap::new(),
            u16_values: HashMap::new(),
            u32_values: HashMap::new(),
            u64_values: HashMap::new(),
            u128_values: HashMap::new(),
            unit_values: HashMap::new(),
            usize_values: HashMap::new(),

            //vector values
            
            string_values: HashMap::new()


        }

    }

    impl_scc_hashmap_all_methods_copy!(bool, bool_values, bool);

    impl_scc_hashmap_all_methods_copy!(char, char_values, char);

    impl_scc_hashmap_all_methods_copy!(f32, f32_values, f32);

    impl_scc_hashmap_all_methods_copy!(f64, f64_values, f64);

    impl_scc_hashmap_all_methods_copy!(i8, i8_values, i8);

    impl_scc_hashmap_all_methods_copy!(i16, i16_values, i16);

    impl_scc_hashmap_all_methods_copy!(i32, i32_values, i32);

    impl_scc_hashmap_all_methods_copy!(i64, i64_values, i64);

    //impl_scc_hashmap_all_methods_copy!(i128, i128_values, i128);

    impl_scc_hashmap_all_methods_copy!(i128, i128_values, I128Scalar);

    impl_scc_hashmap_all_methods_copy!(isize, isize_values, isize);

    impl_scc_hashmap_all_methods_copy!(u8, u8_values, u8);

    impl_scc_hashmap_all_methods_copy!(u16, u16_values, u16);

    impl_scc_hashmap_all_methods_copy!(u32, u32_values, u32);

    impl_scc_hashmap_all_methods_copy!(u64, u64_values, u64);

    //impl_scc_hashmap_all_methods_copy!(u128, u128_values, u128);

    impl_scc_hashmap_all_methods_copy!(u128, u128_values, U128Scalar);

    impl_scc_hashmap_all_methods_copy!(unit_value, unit_values, UnitValue);

    impl_scc_hashmap_all_methods_copy!(usize, usize_values, usize);

    impl_scc_hashmap_all_methods_clone!(string, string_values, String);

    impl_scc_hashmap_all_methods_clone!(whatever, whatever_values, Whatever);

    //Functions etc

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    //impl_scc_hashmap_update_function_call!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    //impl_scc_hashmap_upsert_function_call!(bool, bool_values, bool_bit_and, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_or, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_bit_xor, bool, right_side: bool);

    impl_scc_hashmap_update_upsert_function_calls!(bool, bool_values, bool_not, bool);
    
}


