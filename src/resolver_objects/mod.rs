
//pub mod schema;

pub mod bool_type;

use std::sync::Arc;

pub use bool_type::*;

pub mod char_type;

pub use char_type::*;

pub mod f32_type;

pub use f32_type::*;

pub mod f64_type;

pub use f64_type::*;

pub mod i8_type;

pub use i8_type::*;

pub mod i16_type;

pub use i16_type::*;

pub mod i32_type;

pub use i32_type::*;

pub mod i64_type;

pub use i64_type::*;

pub mod i128_type;

pub use i128_type::*;

pub mod isize_type;

pub use isize_type::*;

pub mod miscellaneous;

pub use miscellaneous::*;

pub mod schema;

pub use schema::*;

pub mod string_type;

pub use string_type::*;

pub mod u8_type;

pub use u8_type::*;

pub mod u16_type;

pub use u16_type::*;

pub mod u32_type;

pub use u32_type::*;

pub mod u64_type;

pub use u64_type::*;

pub mod u128_type;

pub use u128_type::*;

//pub mod unit_value_type;

//pub use unit_value_type::*;

pub mod usize_type;

pub use usize_type::*;

pub mod whatever_type;

pub use whatever_type::*;

pub mod selected_type_type;

pub use selected_type_type::*;

pub mod resolver_object_macros;

pub mod cfgs;

pub use cfgs::*;

pub mod collections;


cfg_if::cfg_if! 
{

    if #[cfg(not(any(feature = "store_aml", feature = "sub_store_aml")))]
    {

        use crate::builds::levels::none::store::Store;
    
        pub type StoreType = Store;

        pub fn new_store() -> StoreType
        {

            StoreType::new()

        }

    }
    else
    {

        #[cfg(feature = "sub_store_aml")]
        use crate::builds::levels::namespace_and_sub_namespace::store::Store;

        cfg_if::cfg_if! 
        {

            if #[cfg(not(any(feature = "all_types", feature = "SelectedTypeIO")))]
            {
            
                pub type StoreType = Store;

                pub fn new_store() -> StoreType
                {

                    StoreType::new()

                }

            }

        }

        cfg_if::cfg_if! 
        {

            if #[cfg(any(feature = "all_types", feature = "SelectedTypeIO"))]
            {

                pub type StoreType = Arc<Store>;

                pub fn new_store() -> StoreType
                {

                    StoreType::new(Store::new())

                }

            }

        }

    }

}
