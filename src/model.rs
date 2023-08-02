/*
use async_graphql::{
    connection::{query, Connection, Edge},
    Context, Enum, Error, Interface, Object, OutputType, Result,
};
*/

use async_graphql::*;

//use crate::async_graphql_value_containers::{*, self};

use crate::types::UnitValue;

//use crate::identifier::*;

//use crate::async_graphql_types::*;

//use crate::builds::store_build::mutex::mtx_ns_scc::Store;

pub struct QueryRoot;

use crate::types::async_graphql_values::{I128Scalar, U128Scalar, Whatever, InputWhatever};
use crate::types::sizes::*;

//use crate::builds::levels::sub_namespace::scc_crate::hashmap_model_macros;

use paste::paste;

use crate::builds::levels::sub_namespace::store::Store;

//QueryRoot/MutationRoot -> dispatch -> store

/*  
https://doc.rust-lang.org/std/primitive.bool.html

https://doc.rust-lang.org/std/primitive.char.html

https://doc.rust-lang.org/std/primitive.f32.html

https://doc.rust-lang.org/std/ops/index.html

https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

https://doc.rust-lang.org/std/cmp/trait.Eq.html

https://doc.rust-lang.org/std/ops/trait.Index.html

https://doc.rust-lang.org/std/ops/trait.IndexMut.html

https://doc.rust-lang.org/std/collections/index.html

https://doc.rust-lang.org/std/vec/struct.Vec.html

https://doc.rust-lang.org/std/vec/index.html

https://doc.rust-lang.org/std/vec/struct.Vec.html

https://doc.rust-lang.org/std/slice/struct.Iter.html

https://doc.rust-lang.org/std/index.html?search=clone

https://doc.rust-lang.org/std/clone/index.html

https://doc.rust-lang.org/std/index.html?search=to_string

https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string

https://redis.io/commands/

https://redis.io/docs/

https://doc.rust-lang.org/std/index.html

https://doc.rust-lang.org/std/num/index.html

https://doc.rust-lang.org/std/ops/index.html

https://doc.rust-lang.org/std/ops/trait.Add.html

https://doc.rust-lang.org/error-index.html#E0277

https://doc.rust-lang.org/rust-by-example/generics/where.html

https://crates.io/crates/async-graphql

https://github.com/async-graphql/async-graphql

https://docs.rs/async-graphql/3.0.38/async_graphql/

https://async-graphql.github.io/async-graphql/en/index.html

https://async-graphql.github.io/async-graphql/en/query_and_mutation.html

https://async-graphql.github.io/async-graphql/en/define_input_object.html

https://async-graphql.github.io/async-graphql/en/define_simple_object.html

https://async-graphql.github.io/async-graphql/en/define_enum.html

https://async-graphql.github.io/async-graphql/en/define_interface.html

https://async-graphql.github.io/async-graphql/en/define_union.html

https://async-graphql.github.io/async-graphql/en/define_input_object.html

https://async-graphql.github.io/async-graphql/en/define_one_of_object.html

https://github.com/graphql/graphql-spec/pull/825

https://async-graphql.github.io/async-graphql/en/default_value.html

*/

//query_methods

//read

//contains

//len

//is_empty

//capacity

//get_all_keys

//mutation_methods

//insert

//update

//upsert

//remove

//clear

#[Object]
impl QueryRoot 
{

    /*
    async fn is_up(&self) -> &str //, ctx: &Context<'_>) -> &str
    {

        "yes"

    }
    */

    async fn test(&self) -> UnitValue
    {

        UnitValue::new()

    }

    //Administration

    async fn size_of_bool(&self) -> usize //, ctx: &Context<'_>
    {

        size_of_bool()

    }

    async fn size_of_char(&self) -> usize //, ctx: &Context<'_>
    {
        
        size_of_char()

    }

    async fn size_of_f32(&self) -> usize
    {

        size_of_f32()

    }

    async fn size_of_f64(&self) -> usize
    {

        size_of_f64()

    }

    async fn size_of_i8(&self) -> usize
    {

        size_of_i8()

    }

    async fn size_of_i16(&self) -> usize
    {

        size_of_i16()

    }

    async fn size_of_i32(&self) -> usize
    {

        size_of_i32()

    }

    async fn size_of_i64(&self) -> usize
    {

        size_of_i64()

    }

    async fn size_of_i128(&self) -> usize
    {

        size_of_i128()

    }
    
    async fn size_of_isize(&self) -> usize
    {

        size_of_isize()

    }

    async fn size_of_u8(&self) -> usize
    {

        size_of_u8()

    }

    async fn size_of_u16(&self) -> usize
    {

        size_of_u16()

    }

    async fn size_of_u32(&self) -> usize
    {

        size_of_u32()

    }

    async fn size_of_u64(&self) -> usize
    {

        size_of_u64()

    }

    async fn size_of_u128(&self) -> usize
    {

        size_of_u128()

    }

    async fn size_of_unit_value(&self) -> usize
    {

        size_of_unit_value()

    }

    async fn size_of_unit_value_combined(&self) -> usize
    {

        size_of_unit_value_combined()

    }

    async fn size_of_usize(&self) ->  usize
    {

        size_of_usize()

    }

    async fn size_of_string(&self) -> usize
    {

        size_of_string()

    }

    //scalar values

    //bool

    pub async fn bool_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_read(key).await

    }

    pub async fn bool_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().bool_contains(key).await

    }

    pub async fn bool_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().bool_len().await

    }

    pub async fn bool_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().bool_is_empty().await

    }

    pub async fn bool_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().bool_get_all_keys().await

    }

    //char

    pub async fn char_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<char>
    {

        ctx.data_unchecked::<Store>().char_read(key).await

    }

    pub async fn char_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().char_contains(key).await

    }

    pub async fn char_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().char_len().await

    }

    pub async fn char_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().char_is_empty().await

    }

    pub async fn char_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().char_get_all_keys().await

    }

    //f32

    pub async fn f32_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<f32>
    {

        ctx.data_unchecked::<Store>().f32_read(key).await

    }

    pub async fn f32_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().f32_contains(key).await

    }

    pub async fn f32_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().f32_len().await

    }

    pub async fn f32_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().f32_is_empty().await

    }

    pub async fn f32_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().f32_get_all_keys().await

    }

    //f64

    pub async fn f64_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<f64>
    {

        ctx.data_unchecked::<Store>().f64_read(key).await

    }

    pub async fn f64_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().f64_contains(key).await

    }

    pub async fn f64_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().f64_len().await

    }

    pub async fn f64_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().f64_is_empty().await

    }

    pub async fn f64_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().f64_get_all_keys().await

    }

    //i8

    pub async fn i8_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<i8>
    {

        ctx.data_unchecked::<Store>().i8_read(key).await

    }

    pub async fn i8_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().i8_contains(key).await

    }

    pub async fn i8_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i8_len().await

    }

    pub async fn i8_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().i8_is_empty().await

    }

    pub async fn i8_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().i8_get_all_keys().await

    }
    
    //i16

    pub async fn i16_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<i16>
    {

        ctx.data_unchecked::<Store>().i16_read(key).await

    }

    pub async fn i16_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().i16_contains(key).await

    }

    pub async fn i16_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i16_len().await

    }

    pub async fn i16_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().i16_is_empty().await

    }

    pub async fn i16_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().i16_get_all_keys().await

    }

    //i32

    pub async fn i32_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<i32>
    {

        ctx.data_unchecked::<Store>().i32_read(key).await

    }

    pub async fn i32_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().i32_contains(key).await

    }

    pub async fn i32_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i32_len().await

    }

    pub async fn i32_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().i32_is_empty().await

    }

    pub async fn i32_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().i32_get_all_keys().await

    }

    //i64

    pub async fn i64_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<i64>
    {

        ctx.data_unchecked::<Store>().i64_read(key).await

    }

    pub async fn i64_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().i64_contains(key).await

    }

    pub async fn i64_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i64_len().await

    }

    pub async fn i64_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().i64_is_empty().await

    }

    pub async fn i64_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().i64_get_all_keys().await

    }

    //i128

    pub async fn i128_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<I128Scalar>
    {

        ctx.data_unchecked::<Store>().i128_read(key).await

    }

    pub async fn i128_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().i128_contains(key).await

    }

    pub async fn i128_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i128_len().await

    }

    pub async fn i128_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().i128_is_empty().await

    }

    pub async fn i128_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().i128_get_all_keys().await

    }

    //isize

    pub async fn isize_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<isize>
    {

        ctx.data_unchecked::<Store>().isize_read(key).await

    }

    pub async fn isize_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().isize_contains(key).await

    }

    pub async fn isize_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().isize_len().await

    }

    pub async fn isize_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().isize_is_empty().await

    }

    pub async fn isize_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().isize_get_all_keys().await

    }

    //

    //u8

    pub async fn u8_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<u8>
    {

        ctx.data_unchecked::<Store>().u8_read(key).await

    }

    pub async fn u8_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().u8_contains(key).await

    }

    pub async fn u8_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u8_len().await

    }

    pub async fn u8_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().u8_is_empty().await

    }

    pub async fn u8_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().u8_get_all_keys().await

    }

    //u16

    pub async fn u16_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<u16>
    {

        ctx.data_unchecked::<Store>().u16_read(key).await

    }

    pub async fn u16_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().u16_contains(key).await

    }

    pub async fn u16_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u16_len().await

    }

    pub async fn u16_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().u16_is_empty().await

    }

    pub async fn u16_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().u16_get_all_keys().await

    }
    
    //u32

    pub async fn u32_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<u32>
    {

        ctx.data_unchecked::<Store>().u32_read(key).await

    }

    pub async fn u32_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().u32_contains(key).await

    }

    pub async fn u32_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u32_len().await

    }

    pub async fn u32_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().u32_is_empty().await

    }

    pub async fn u32_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().u32_get_all_keys().await

    }

    //u64

    pub async fn u64_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<u64>
    {

        ctx.data_unchecked::<Store>().u64_read(key).await

    }

    pub async fn u64_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().u64_contains(key).await

    }

    pub async fn u64_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u64_len().await

    }

    pub async fn u64_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().u64_is_empty().await

    }

    pub async fn u64_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().u64_get_all_keys().await

    }

    //u128

    pub async fn u128_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<U128Scalar>
    {

        ctx.data_unchecked::<Store>().u128_read(key).await

    }

    pub async fn u128_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().u128_contains(key).await

    }

    pub async fn u128_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u128_len().await

    }

    pub async fn u128_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().u128_is_empty().await

    }

    pub async fn u128_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().u128_get_all_keys().await

    }

    //usize

    pub async fn usize_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<usize>
    {

        ctx.data_unchecked::<Store>().usize_read(key).await

    }

    pub async fn usize_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().usize_contains(key).await

    }

    pub async fn usize_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().usize_len().await

    }

    pub async fn usize_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().usize_is_empty().await

    }

    pub async fn usize_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().usize_get_all_keys().await

    }

    //UnitValue

    pub async fn unit_value_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().unit_value_read(key).await

    }

    pub async fn unit_value_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().unit_value_contains(key).await

    }

    pub async fn unit_value_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().unit_value_len().await

    }

    pub async fn unit_value_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().unit_value_is_empty().await

    }

    pub async fn unit_value_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().unit_value_get_all_keys().await

    }

    //String

    pub async fn string_read(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<String>
    {

        ctx.data_unchecked::<Store>().string_read(key).await

    }

    pub async fn string_contains(&self, ctx: &Context<'_>, key: String) -> bool
    {

        ctx.data_unchecked::<Store>().string_contains(key).await

    }

    pub async fn string_len(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().string_len().await

    }

    pub async fn string_is_empty(&self, ctx: &Context<'_>) -> bool
    {

        ctx.data_unchecked::<Store>().string_is_empty().await

    }

    pub async fn string_get_all_keys(&self, ctx: &Context<'_>) -> Vec<String>
    {

        ctx.data_unchecked::<Store>().string_get_all_keys().await

    }

    /*
    crate::impl_scc_hashmap_model_query_methods!(bool, String, bool);

    crate::impl_scc_hashmap_model_bool_mutation_fn_methods!(String);

    //

    crate::impl_scc_hashmap_model_query_methods!(char, String, char);

    crate::impl_scc_hashmap_model_query_methods!(f32, String, f32);

    crate::impl_scc_hashmap_model_query_methods!(f64, String, f64);

    crate::impl_scc_hashmap_model_query_methods!(i8, String, i8);

    crate::impl_scc_hashmap_model_query_methods!(i16, String, i16);

    crate::impl_scc_hashmap_model_query_methods!(i32, String, i32);

    crate::impl_scc_hashmap_model_query_methods!(i64, String, i64);

    //crate::impl_scc_hashmap_model_query_methods!(i128, String, i128);

    crate::impl_scc_hashmap_model_query_methods!(i128, String, I128Scalar);

    crate::impl_scc_hashmap_model_query_methods!(isize, String, isize);

    crate::impl_scc_hashmap_model_query_methods!(u8, String, u8);

    crate::impl_scc_hashmap_model_query_methods!(u16, String, u16);

    crate::impl_scc_hashmap_model_query_methods!(u32, String, u32);

    crate::impl_scc_hashmap_model_query_methods!(u64, String, u64);

    //crate::impl_scc_hashmap_model_query_methods!(u128, String, u128);

    crate::impl_scc_hashmap_model_query_methods!(u128, String, U128Scalar);

    crate::impl_scc_hashmap_model_query_methods!(unit_value, String, UnitValue);

    crate::impl_scc_hashmap_model_query_methods!(usize, String, usize);

    //vector values

    crate::impl_scc_hashmap_model_query_methods!(string, String, String);
    */

    //

    /*
    the trait bound `(): OutputType` is not satisfied
    the following other types implement trait `OutputType`:
    &'a [T]
    &'impl0 T
    AnyObject
    BTreeMap<K, V>
    BTreeSet<T>
    BoolValue
    Box<T>
    Box<[T]>
    and 84 othersrustcE0277

    async fn get_unit(&self) -> ()
    {

        

    }
    */


}

pub struct MutationRoot;

#[Object]
impl MutationRoot
{

    /*
    async fn get_u8(&self, ctx: &Context<'_>) -> Result<u8>
    {

        Ok(0)

        //ctx.data_unchecked::<Store>().get_u8(identifier).await

    }

    async fn get_123(&self, ctx: &Context<'_>) -> Result<String>
    {

        Ok("123".to_string())

        //ctx.data_unchecked::<Store>().get_u8(identifier).await

    }
    */

    //scalar values

    //bool

    pub async fn bool_insert(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().bool_insert(key, value).await

    }

    pub async fn bool_update(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().bool_update(key, value).await

    }

    pub async fn bool_upsert(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<UnitValue>
    {

        //self.$field.update_fn(key, $updater_fn_name).await

        ctx.data_unchecked::<Store>().bool_upsert(key, value).await

    }

    pub async fn bool_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().bool_remove(key).await

    }

    pub async fn bool_clear(&self, ctx: &Context<'_>) -> usize //async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().bool_clear().await

    }

    //fns

    //with param

    pub async fn bool_bit_and_update_fn_param(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_bit_and_update_fn_param(key, value).await

    }

    pub async fn bool_bit_or_update_fn_param(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_bit_or_update_fn_param(key, value).await

    }

    pub async fn bool_bit_xor_update_fn_param(&self, ctx: &Context<'_>, key: String, value: bool) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_bit_xor_update_fn_param(key, value).await

    }

    //on self

    pub async fn bool_bit_and_self_update_fn(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<bool>
    {

        //self.$field.update_fn(key, $updater_fn_name).await

        ctx.data_unchecked::<Store>().bool_bit_and_self_update_fn(key).await

    }

    pub async fn bool_bit_or_self_update_fn(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_bit_or_self_update_fn(key).await

    }

    pub async fn bool_bit_xor_self_update_fn(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<bool>
    {

        ctx.data_unchecked::<Store>().bool_bit_xor_self_update_fn(key).await

    }

    //char

    pub async fn char_insert(&self, ctx: &Context<'_>, key: String, value: char) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().char_insert(key, value).await

    }

    pub async fn char_update(&self, ctx: &Context<'_>, key: String, value: char) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().char_update(key, value).await

    }

    pub async fn char_upsert(&self, ctx: &Context<'_>, key: String, value: char) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().char_upsert(key, value).await

    }

    pub async fn char_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().char_remove(key).await

    }

    pub async fn char_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().char_clear().await

    }

    //f32

    pub async fn f32_insert(&self, ctx: &Context<'_>, key: String, value: f32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f32_insert(key, value).await

    }

    pub async fn f32_update(&self, ctx: &Context<'_>, key: String, value: f32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f32_update(key, value).await

    }

    pub async fn f32_upsert(&self, ctx: &Context<'_>, key: String, value: f32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f32_upsert(key, value).await

    }

    pub async fn f32_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f32_remove(key).await

    }

    pub async fn f32_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().f32_clear().await

    }

    //f64

    pub async fn f64_insert(&self, ctx: &Context<'_>, key: String, value: f64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f64_insert(key, value).await

    }

    pub async fn f64_update(&self, ctx: &Context<'_>, key: String, value: f64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f64_update(key, value).await

    }

    pub async fn f64_upsert(&self, ctx: &Context<'_>, key: String, value: f64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f64_upsert(key, value).await

    }

    pub async fn f64_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().f64_remove(key).await

    }

    pub async fn f64_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().f64_clear().await

    }

    //i8

    pub async fn i8_insert(&self, ctx: &Context<'_>, key: String, value: i8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i8_insert(key, value).await

    }

    pub async fn i8_update(&self, ctx: &Context<'_>, key: String, value: i8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i8_update(key, value).await

    }

    pub async fn i8_upsert(&self, ctx: &Context<'_>, key: String, value: i8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i8_upsert(key, value).await

    }

    pub async fn i8_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i8_remove(key).await

    }

    pub async fn i8_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i8_clear().await

    }

    //i16

    pub async fn i16_insert(&self, ctx: &Context<'_>, key: String, value: i16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i16_insert(key, value).await

    }

    pub async fn i16_update(&self, ctx: &Context<'_>, key: String, value: i16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i16_update(key, value).await

    }

    pub async fn i16_upsert(&self, ctx: &Context<'_>, key: String, value: i16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i16_upsert(key, value).await

    }

    pub async fn i16_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i16_remove(key).await

    }

    pub async fn i16_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i16_clear().await

    }

    //i32

    pub async fn i32_insert(&self, ctx: &Context<'_>, key: String, value: i32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i32_insert(key, value).await

    }

    pub async fn i32_update(&self, ctx: &Context<'_>, key: String, value: i32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i32_update(key, value).await

    }

    pub async fn i32_upsert(&self, ctx: &Context<'_>, key: String, value: i32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i32_upsert(key, value).await

    }

    pub async fn i32_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i32_remove(key).await

    }

    pub async fn i32_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i32_clear().await

    }

    //i64

    pub async fn i64_insert(&self, ctx: &Context<'_>, key: String, value: i64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i64_insert(key, value).await

    }

    pub async fn i64_update(&self, ctx: &Context<'_>, key: String, value: i64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i64_update(key, value).await

    }

    pub async fn i64_upsert(&self, ctx: &Context<'_>, key: String, value: i64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i64_upsert(key, value).await

    }

    pub async fn i64_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i64_remove(key).await

    }

    pub async fn i64_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i64_clear().await

    }

    //i128

    pub async fn i128_insert(&self, ctx: &Context<'_>, key: String, value: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i128_insert(key, value).await

    }

    pub async fn i128_update(&self, ctx: &Context<'_>, key: String, value: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i128_update(key, value).await

    }

    pub async fn i128_upsert(&self, ctx: &Context<'_>, key: String, value: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i128_upsert(key, value).await

    }

    pub async fn i128_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().i128_remove(key).await

    }

    pub async fn i128_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().i128_clear().await

    }

    //isize

    pub async fn isize_insert(&self, ctx: &Context<'_>, key: String, value: isize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().isize_insert(key, value).await

    }

    pub async fn isize_update(&self, ctx: &Context<'_>, key: String, value: isize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().isize_update(key, value).await

    }

    pub async fn isize_upsert(&self, ctx: &Context<'_>, key: String, value: isize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().isize_upsert(key, value).await

    }

    pub async fn isize_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().isize_remove(key).await

    }

    pub async fn isize_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().isize_clear().await

    }

    //u8

    pub async fn u8_insert(&self, ctx: &Context<'_>, key: String, value: u8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u8_insert(key, value).await

    }

    pub async fn u8_update(&self, ctx: &Context<'_>, key: String, value: u8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u8_update(key, value).await

    }

    pub async fn u8_upsert(&self, ctx: &Context<'_>, key: String, value: u8) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u8_upsert(key, value).await

    }

    pub async fn u8_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u8_remove(key).await

    }

    pub async fn u8_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u8_clear().await

    }

    //u16

    pub async fn u16_insert(&self, ctx: &Context<'_>, key: String, value: u16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u16_insert(key, value).await

    }

    pub async fn u16_update(&self, ctx: &Context<'_>, key: String, value: u16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u16_update(key, value).await

    }

    pub async fn u16_upsert(&self, ctx: &Context<'_>, key: String, value: u16) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u16_upsert(key, value).await

    }

    pub async fn u16_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u16_remove(key).await

    }

    pub async fn u16_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u16_clear().await

    }

    //u32

    pub async fn u32_insert(&self, ctx: &Context<'_>, key: String, value: u32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u32_insert(key, value).await

    }

    pub async fn u32_update(&self, ctx: &Context<'_>, key: String, value: u32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u32_update(key, value).await

    }

    pub async fn u32_upsert(&self, ctx: &Context<'_>, key: String, value: u32) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u32_upsert(key, value).await

    }

    pub async fn u32_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u32_remove(key).await

    }

    pub async fn u32_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u32_clear().await

    }

    //u64

    pub async fn u64_insert(&self, ctx: &Context<'_>, key: String, value: u64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u64_insert(key, value).await

    }

    pub async fn u64_update(&self, ctx: &Context<'_>, key: String, value: u64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u64_update(key, value).await

    }

    pub async fn u64_upsert(&self, ctx: &Context<'_>, key: String, value: u64) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u64_upsert(key, value).await

    }

    pub async fn u64_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u64_remove(key).await

    }

    pub async fn u64_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u64_clear().await

    }
    
    //u128

    pub async fn u128_insert(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u128_insert(key, value).await

    }

    pub async fn u128_update(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u128_update(key, value).await

    }

    pub async fn u128_upsert(&self, ctx: &Context<'_>, key: String, value: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u128_upsert(key, value).await

    }

    pub async fn u128_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().u128_remove(key).await

    }

    pub async fn u128_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().u128_clear().await

    }

    //UnitValue

    pub async fn unit_value_insert(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<Store>().unit_value_insert(key, UnitValue::new()).await

    }

    pub async fn unit_value_update(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<Store>().unit_value_update(key, UnitValue::new()).await

    }

    pub async fn unit_value_upsert(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue> //, value: UnitValue
    {

        ctx.data_unchecked::<Store>().unit_value_upsert(key, UnitValue::new()).await

    }

    pub async fn unit_value_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().unit_value_remove(key).await

    }

    pub async fn unit_value_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().unit_value_clear().await

    }

    //usize

    pub async fn usize_insert(&self, ctx: &Context<'_>, key: String, value: usize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().usize_insert(key, value).await

    }

    pub async fn usize_update(&self, ctx: &Context<'_>, key: String, value: usize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().usize_update(key, value).await

    }

    pub async fn usize_upsert(&self, ctx: &Context<'_>, key: String, value: usize) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().usize_upsert(key, value).await

    }

    pub async fn usize_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().usize_remove(key).await

    }

    pub async fn usize_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().usize_clear().await

    }

    //String

    pub async fn string_insert(&self, ctx: &Context<'_>, key: String, value: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().string_insert(key, value).await

    }

    pub async fn string_update(&self, ctx: &Context<'_>, key: String, value: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().string_update(key, value).await

    }

    pub async fn string_upsert(&self, ctx: &Context<'_>, key: String, value: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().string_upsert(key, value).await

    }

    pub async fn string_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().string_remove(key).await

    }

    pub async fn string_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().string_clear().await

    }

    //Whatever

    pub async fn whatever_insert(&self, ctx: &Context<'_>, key: String, value: InputWhatever) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().whatever_insert(key, value.into()).await

    }

    pub async fn whatever_update(&self, ctx: &Context<'_>, key: String, value: InputWhatever) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().whatever_update(key, value.into()).await

    }

    pub async fn whatever_upsert(&self, ctx: &Context<'_>, key: String, value: InputWhatever) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().whatever_upsert(key, value.into()).await

    }

    pub async fn whatever_remove(&self, ctx: &Context<'_>, key: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().whatever_remove(key).await

    }

    pub async fn whatever_clear(&self, ctx: &Context<'_>) -> usize
    {

        ctx.data_unchecked::<Store>().whatever_clear().await

    }

    //crate::impl_scc_hashmap_model_mutation_methods!(bool, String, bool);

    /*
    crate::impl_store_container_insert!(bool, String, bool);

    crate::impl_store_container_update!(bool, String, bool);

    crate::impl_store_container_upsert!(bool, String, bool);

    crate::impl_store_container_remove!(bool, String);

    crate::impl_store_container_clear!(bool);
    */

}


    //pointer?

    //reference?

    /*
    async fn get_u8(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<u8>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_u8(identifier).await

    }

    async fn get_u16(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<u16>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_u16(identifier).await

    }

    async fn get_u32(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<u32> //ctx: &Context<'_>, input: u32) -> u32
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_u32(identifier).await

    }

    async fn get_u64(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<u64>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_u64(identifier).await

    }

    async fn get_u128(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<U128Scalar>
    {

        //Ok(async_graphql_value_containers::U128Scalar(0))

        ctx.data_unchecked::<Store>().get_u128(identifier).await

    }

    async fn get_usize(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<usize>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_usize(identifier).await

    }

    //Non=Primitive Types

    //string value
    async fn get_string(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<String> //<&str>
    {

        //May have to clone it

        //Ok(" ")

        ctx.data_unchecked::<Store>().get_string(identifier).await

    }
    */

    /*
    //a string repesentation of something
    async fn as_string(&self, identifier: Identifier) -> Result<&str>
    {

        //May have to clone it

        Ok(" ")

    }
    */

    /* Test */
    //-------------------------------------------------------------------------------------------------------------------

    /*
    async fn get_arc_string(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<std::sync::Arc<String>>
    {

        let res = std::sync::Arc::new(" ".to_string());

        Ok(res)

    }

    async fn get_arc_i32(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<std::sync::Arc<i32>>
    {

        let res = std::sync::Arc::new(1);

        Ok(res)

    }
    */

    //-------------------------------------------------------------------------------------------------------------------
    /* Test */

    /*
    async fn get_arc_mutex_string(&self, identifier: Identifier) -> Result<std::sync::Arc<std::sync::Mutex<String>>>
    {

        let res = std::sync::Arc::new(std::sync::Mutex::new(" ".to_string()));

        Ok(res)

    }
    */



    /*
    async fn get_arc_mutex_string(&self, identifier: Identifier) -> std::sync::Arc<std::sync::Mutex<String>> //Result<std::sync::MutexGuard<String>> // , std::sync::PoisonError<std::sync::MutexGuard<String>>> //Result<&String>
    {

        let res = std::sync::Arc::new(std::sync::Mutex::new(" ".to_string()));

        //let res_lock;

        unsafe
        {

            MTX_TEST = Some(res);

            //res_lock = MTX_TEST.as_ref().unwrap().lock();

            MTX_TEST.as_ref().unwrap().clone()

        }

        //let res_lock = res.lock();

        //Ok(res_lock.unwrap())

        /*
        match res_lock
        {

            Ok(res) => Ok(res),
            Err(_) => todo!(),

        }
        */  

        //Ok(res_lock)

    }
    */

//}

/*
static mut MTX_TEST: Option<std::sync::Arc<std::sync::Mutex<String>>> = None;

pub struct MutationRoot;

#[Object] //<'ctx>
impl MutationRoot //<'ctx>
{

    async fn create_namespace(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().create_namespace(name).await

    }

    async fn drop_namespace(&self, ctx: &Context<'_>, name: String) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().drop_namespace(name).await

    }

    //ops then direct access/implementations

    async fn create_stored_object(&self, ctx: &Context<'_>, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue> //<&str> //<'ctx>
    {

        //let store = ctx.data::<Store>().unwrap();
        
        //store.create_stored_object(the_type).await

        //Ok("Success!")

        ctx.data_unchecked::<Store>().create_stored_object(identifier, the_type).await     

    }
    
    async fn create_stored_object_mut(&self, ctx: &Context<'_>, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().create_stored_object_mut(identifier, the_type).await    

        //Ok("Success!")

    }

    async fn create_generic_stored_object(&self, ctx: &Context<'_>, identifier: Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().create_generic_stored_object(identifier, the_type).await  

        //Ok("Success!")

    }

    async fn create_generic_stored_object_mut(&self, ctx: &Context<'_>, identifier: Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        ctx.data_unchecked::<Store>().create_generic_stored_object_mut(identifier, the_type).await  

        //Ok("Success!")

    }

    async fn set_bool(&self, ctx: &Context<'_>, identifier: Identifier, input: bool) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_bool(identifier, input).await

    }

    async fn set_char(&self, ctx: &Context<'_>, identifier: Identifier, input: char) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_char(identifier, input).await

    }

    async fn set_f32(&self, ctx: &Context<'_>, identifier: Identifier, input: f32) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_f32(identifier, input).await

    }

    async fn set_f64(&self, ctx: &Context<'_>, identifier: Identifier, input: f64) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_f64(identifier, input).await

    }

    async fn set_i8(&self, ctx: &Context<'_>, identifier: Identifier, input: i8) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_i8(identifier, input).await

    }

    async fn set_i16(&self, ctx: &Context<'_>, identifier: Identifier, input: i16) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_i16(identifier, input).await

    }

    async fn set_i32(&self, ctx: &Context<'_>, identifier: Identifier, input: i32) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_i32(identifier, input).await

    }

    async fn set_i64(&self, ctx: &Context<'_>, identifier: Identifier, input: i64) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_i64(identifier, input).await

    }

    async fn set_i128(&self, ctx: &Context<'_>, identifier: Identifier, input: I128Scalar) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        //Ok(I128::default())

        ctx.data_unchecked::<Store>().set_i128(identifier, input).await

    }

    async fn set_isize(&self, ctx: &Context<'_>, identifier: Identifier, input: isize) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_isize(identifier, input).await

    }

    //pointer?

    //reference?

    async fn set_u8(&self, ctx: &Context<'_>, identifier: Identifier, input: u8) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_u8(identifier, input).await

    }

    async fn set_u16(&self, ctx: &Context<'_>, identifier: Identifier, input: u16) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_u16(identifier, input).await

    }

    async fn set_u32(&self, ctx: &Context<'_>, identifier: Identifier, input: u32) -> Result<UnitValue> //ctx: &Context<'_>, input: u32) -> u32
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_u32(identifier, input).await

    }

    async fn set_u64(&self, ctx: &Context<'_>, identifier: Identifier, input: u64) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_u64(identifier, input).await

    }

    async fn set_u128(&self, ctx: &Context<'_>, identifier: Identifier, input: U128Scalar) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_u128(identifier, input).await

    }

    async fn set_usize(&self, ctx: &Context<'_>, identifier: Identifier, input: usize) -> Result<UnitValue>
    {

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_usize(identifier, input).await

    }

    //Non=Primitive Types

    //string value
    async fn set_string(&self, ctx: &Context<'_>, identifier: Identifier, input: String) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_string(identifier, input).await

    }

    async fn set_value(&self, ctx: &Context<'_>, identifier: Identifier, input: AnyInputObject) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        //Ok(UnitValue::new())

        ctx.data_unchecked::<Store>().set_value(identifier, input).await

    }
        
}
*/
