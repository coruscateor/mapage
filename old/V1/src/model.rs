/*
use async_graphql::{
    connection::{query, Connection, Edge},
    Context, Enum, Error, Interface, Object, OutputType, Result,
};
*/

use async_graphql::*;

use crate::async_graphql_value_containers::{*, self};

use crate::consts::UnitValue;

use crate::identifier::*;

use crate::async_graphql_types::*;

use crate::builds::store_build::mutex::mtx_ns_scc::Store;

pub struct QueryRoot;

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

    async fn contains_namespace(&self, ctx: &Context<'_>, name: String) -> bool
    {

        ctx.data_unchecked::<Store>().contains_namespace(name).await  

    }

    pub fn get_all_namespace_names(&self) -> HashSet<String>
    {
        
        let mut set = HashSet::new();

        let mut set_ref = &mut set;

        self.namespaces.for_each(|name, _| {

            set_ref.insert(name.clone());

        });

        set

    }

    pub fn count_default_namespaced_values(&self) -> usize
    {

        let lk = self.default_namespace.lock().unwrap();

        lk.get_count()

    }

    pub fn count_namespaced_values(&self, name: String) -> Option<usize>
    {

        let res = self.namespaces.read(&name, |name, mtx| {

            let ns = mtx.lock().unwrap();

            ns.get_count()

        });

        res

    }

    pub fn get_default_namespace_value_names(&self) -> Vec<String>
    {

        let lk = self.default_namespace.lock().unwrap();

        lk.get_value_names()

    }

    pub fn get_namespace_names(&self, name: String) -> Option<Vec<String>>
    {

        self.namespaces.read(&name, |_, mtx|
        {

            let ns = mtx.lock().unwrap();
            
            ns.get_value_names()

        })

    }

    pub fn namespaces_count(&self) -> usize
    {

        self.namespaces.len()

    }

    //ops then direct access/implementations

    async fn get_value(&self, ctx: &Context<'_>, identifier: Identifier) -> async_graphql::Result<Option<AnyObject>>
    {

        ctx.data_unchecked::<Store>().get_value(identifier).await  

        //Ok(Some(AnyObject::Bool(Bool::default())))

    }

    /*
    async fn get_value(&self, ctx: &Context<'_>, identifier: Identifier, output_format: String) -> async_graphql::Result<String>
    {

        Ok(" ".to_string())

    }
    */

    //Primitive Types
    
    async fn get_bool(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<bool>
    {

        //Ok(Bool::default())

        //Ok(true)

        ctx.data_unchecked::<Store>().get_bool(identifier).await

    }

    async fn get_char(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<char>
    {

        //Ok(Char::default())

        //Ok(' ')
        
        ctx.data_unchecked::<Store>().get_char(identifier).await

    }

    async fn get_f32(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<f32>
    {

        //Ok(0.0)

        ctx.data_unchecked::<Store>().get_f32(identifier).await

    }

    async fn get_f64(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<f64>
    {

        //Ok(0.0)

        ctx.data_unchecked::<Store>().get_f64(identifier).await

    }

    async fn get_i8(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<i8>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_i8(identifier).await

    }

    async fn get_i16(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<i16>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_i16(identifier).await

    }

    async fn get_i32(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<i32>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_i32(identifier).await

    }

    async fn get_i64(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<i64>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_i64(identifier).await

    }

    async fn get_i128(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<I128Scalar> //<i128>
    {

        //Ok(async_graphql_value_containers::I128Scalar(0))

        ctx.data_unchecked::<Store>().get_i128(identifier).await

    }

    async fn get_isize(&self, ctx: &Context<'_>, identifier: Identifier) -> Result<isize>
    {

        //Ok(0)

        ctx.data_unchecked::<Store>().get_isize(identifier).await

    }

    //pointer?

    //reference?

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

}

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