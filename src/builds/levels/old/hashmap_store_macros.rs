use crate::types::unit_value::UnitValue;

use paste::paste;

use async_graphql::{Result, Error, ErrorExtensions};

use crate::errors::*;

use crate::types::async_graphql_values::{I128Scalar, U128Scalar};

//https://docs.rs/scc/0.11.1/scc/hash_map/struct.HashMap.html

//insert_async

#[macro_export]
macro_rules! impl_scc_hashmap_insert
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _insert>](&self, key: String, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.insert_async(key, value);

                if let Err(_) = res.await
                {

                    return invalid_operation();

                }

                Ok(UnitValue::new())

            }

        }

    }

}

//call methods...

//update_async

//returns unit and not the value_type because of vector types

#[macro_export]
macro_rules! impl_scc_hashmap_update
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _update>](&self, key: String, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.update_async(&key, |_, v| { *v = value; /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    }

}

//upsert_async

#[macro_export]
macro_rules! impl_scc_hashmap_upsert
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        paste! {

            pub async fn [<$label _upsert>](&self, key: String, value: $value_type) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.upsert_async(key, || $value_type::default(), |_, v| { *v = value; /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    }

}

//remove_async

#[macro_export]
macro_rules! impl_scc_hashmap_remove
{

    ($label:ident, $field:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _remove>](&self, key: &String) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.remove_async(key);

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    }

}

//remove_if_async

//read_async - copy

#[macro_export]
macro_rules! impl_scc_hashmap_read_copy
{

    ($label:ident, $field:ident, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: &String) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| *v);

                if let Some(val) = res.await
                {

                    return Ok(val);

                }
                
                invalid_operation()

            }

        }

    }

}

//read_async - clone

#[macro_export]
macro_rules! impl_scc_hashmap_read_clone
{

    ($label:ident, $field:ident, $return_type:ty) =>
    {

        paste! {

            pub async fn [<$label _read>](&self, key: &String) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| v.clone());

                if let Some(val) = res.await
                {

                    return Ok(val);

                }
                
                invalid_operation()

            }

        }

    }

}

//contains_async

#[macro_export]
macro_rules! impl_scc_hashmap_contains
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _contains>](&self, key: &String) -> bool
            {

                self.$field.contains_async(key).await

            }

        }

    }

}

//scan_async

//for_each_async

//retain_async

//clear_async

#[macro_export]
macro_rules! impl_scc_hashmap_clear
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _clear>](&self) -> usize
            {

                self.$field.clear_async().await

            }

        }

    }

}

//len

#[macro_export]
macro_rules! impl_scc_hashmap_len
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _len>](&self) -> usize
            {

                self.$field.len()

            }

        }

    }

}

//is_empty

#[macro_export]
macro_rules! impl_scc_hashmap_is_empty
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _is_empty>](&self) -> bool
            {

                self.$field.is_empty()

            }

        }

    }

}

//capacity

#[macro_export]
macro_rules! impl_scc_hashmap_capacity
{

    ($label:ident, $field:ident) =>
    {

        paste! {

            pub async fn [<$label _capacity>](&self) -> usize
            {

                self.$field.capacity()

            }

        }

    }

}

//

//Copy

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_copy
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_insert!($label, $field, $value_type);

        crate::impl_scc_hashmap_update!($label, $field, $value_type);

        crate::impl_scc_hashmap_upsert!($label, $field, $value_type);

        crate::impl_scc_hashmap_remove!($label, $field);

        crate::impl_scc_hashmap_read_copy!($label, $field, $value_type);

        crate::impl_scc_hashmap_contains!($label, $field);

        crate::impl_scc_hashmap_clear!($label, $field);

        crate::impl_scc_hashmap_len!($label, $field);

        crate::impl_scc_hashmap_is_empty!($label, $field);

        crate::impl_scc_hashmap_capacity!($label, $field);

    }

}

//Clone

#[macro_export]
macro_rules! impl_scc_hashmap_all_methods_clone
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_insert!($label, $field, $value_type);

        crate::impl_scc_hashmap_update!($label, $field, $value_type);

        crate::impl_scc_hashmap_upsert!($label, $field, $value_type);

        crate::impl_scc_hashmap_remove!($label, $field);

        crate::impl_scc_hashmap_read_clone!($label, $field, $value_type);

        crate::impl_scc_hashmap_contains!($label, $field);

        crate::impl_scc_hashmap_clear!($label, $field);

        crate::impl_scc_hashmap_len!($label, $field);

        crate::impl_scc_hashmap_is_empty!($label, $field);

        crate::impl_scc_hashmap_capacity!($label, $field);

    }

}

//get all

//call methods

//scalar type method calls

//update_async

#[macro_export]
macro_rules! impl_scc_hashmap_update_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { v.$method(); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { v.$method() });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.update_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.update_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//upsert_async

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method(); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method() });

                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { v.$method($($parameter_name: $name_type,)*) });

                Ok(UnitValue::new())

            }

        }

    };

}

//

#[macro_export]
macro_rules! impl_scc_hashmap_update_upsert_method_calls
{

    ($label:ident, $field:ident, $method:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $return_type:ty) =>
    {
        
        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type, $return_type);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type,$return_type);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type, $($parameter_name:ident: $name_type:ty)*);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type, $($parameter_name:ident: $name_type:ty)*);

    };
    ($label:ident, $field:ident, $method:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_method_call($label, $field, $method, $value_type,$return_type, $($parameter_name:ident: $name_type:ty)*);

        crate::impl_scc_hashmap_uppsert_method_call($label, $field, $method, $value_type, $return_type, $($parameter_name:ident: $name_type:ty)*);

    };

}

//read_async

#[macro_export]
macro_rules! impl_scc_hashmap_read_method_call
{

    ($label:ident, $field:ident, $method:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { v.$method(); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { v.$method() });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.read_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $method:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $method>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| { v.$method($($parameter_name: $name_type,)*) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//scalar type function calls

//update_async

#[macro_export]
macro_rules! impl_scc_hashmap_update_function_call
{

    ($label:ident, $field:ident, $function:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { $function(v); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.update_async(key, |_, v| { $function(v) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.update_async(key, |_, v| { $function(v, $($parameter_name,)*); /*()*/ }); //: $name_type

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.update_async(key, |_, v| { $function(v, $($parameter_name,)*) }); //: $name_type

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//upsert_async

#[macro_export]
macro_rules! impl_scc_hashmap_upsert_function_call
{

    ($label:ident, $field:ident, $function:ident, $value_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v); /*()*/ });
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v) });

                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v, $($parameter_name,)*); /*()*/ }).await; //: $name_type
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _upsert_call_ $function>](&self, key: String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                self.$field.upsert_async(key, || $value_type::default(), |_, v| { $function(v, $($parameter_name,)*) }).await; //: $name_type

                Ok(UnitValue::new())

            }

        }

    };

}

//

#[macro_export]
macro_rules! impl_scc_hashmap_update_upsert_function_calls
{

    ($label:ident, $field:ident, $function:ident, $value_type:ty) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty) =>
    {
        
        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $return_type);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $return_type);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $($parameter_name: $name_type)*);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $($parameter_name: $name_type)*);

    };
    ($label:ident, $field:ident, $function:ident, $value_type:ty, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        crate::impl_scc_hashmap_update_function_call!($label, $field, $function, $value_type, $return_type, $($parameter_name: $name_type)*);

        crate::impl_scc_hashmap_upsert_function_call!($label, $field, $function, $value_type, $return_type, $($parameter_name: $name_type)*);

    };

}

//read_async

#[macro_export]
macro_rules! impl_scc_hashmap_read_function_call
{

    ($label:ident, $field:ident, $function:ident) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _read_call_ $function>](&self, key: &String) -> async_graphql::Result<UnitValue> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { $function(v); /*()*/ });

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty) => //, $value_type:ty
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, ) -> async_graphql::Result<$return_type> //value: $value_type
            {

                let res = self.$field.read_async(key, |_, v| { $function(v) });

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _update_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.read_async(key, |_, v| { $function(v, $($parameter_name)*); /*()*/ }); //: $name_type,

                if let None = res.await
                {

                    return invalid_operation();

                }
                
                Ok(UnitValue::new())

            }

        }

    };
    ($label:ident, $field:ident, $function:ident, $return_type:ty, $($parameter_name:ident: $name_type:ty)*) =>
    {

        paste! {

            pub async fn [<$label _read_call_ $function>](&self, key: &String, $($parameter_name: $name_type,)*) -> async_graphql::Result<$return_type>
            {

                let res = self.$field.read_async(key, |_, v| { $function(v, $($parameter_name,)*) }); //: $name_type

                if let Some(val) = res.await
                {

                    return Ok(val);

                }

                invalid_operation()

            }

        }

    };

}

//extract result if method returns result



//vector type method calls


