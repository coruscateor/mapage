use crate::{stored_object::*, all_types::{Type, AllTypes}, errors::{invalid_operation_instance_is_immuatable, invalid_operation}, binary_operations::Bop, async_graphql_value_containers::{NumericOrBool, AnyObject, I128Scalar, U128Scalar, AnyInputObject}, uniary_operations::Uop, identifier::*, consts::UnitValue};

use async_trait::async_trait;



pub trait StorageContainerGetters: Send
{

    fn get_type(&self) -> Type; //, all_types: &AllTypes

    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        invalid_operation()
        
    }

    fn get_char(&self) -> async_graphql::Result<char>
    {

        invalid_operation()
        
    }

    fn get_f32(&self) -> async_graphql::Result<f32>
    {

        invalid_operation()
        
    }

    fn get_f64(&self) -> async_graphql::Result<f64>
    {

        invalid_operation()
        
    }

    fn get_i8(&self) -> async_graphql::Result<i8>
    {

        invalid_operation()
        
    }

    fn get_i16(&self) -> async_graphql::Result<i16>
    {

        invalid_operation()
        
    }

    fn get_i32(&self) -> async_graphql::Result<i32>
    {

        invalid_operation()
        
    }

    fn get_i64(&self) -> async_graphql::Result<i64>
    {

        invalid_operation()
        
    }

    fn get_i128(&self) -> async_graphql::Result<I128Scalar>
    {

        invalid_operation()
        
    }

    fn get_isize(&self) -> async_graphql::Result<isize>
    {

        invalid_operation()
        
    }

    //

    fn get_u8(&self) -> async_graphql::Result<u8>
    {

        invalid_operation()
        
    }

    fn get_u16(&self) -> async_graphql::Result<u16>
    {

        invalid_operation()
        
    }

    fn get_u32(&self) -> async_graphql::Result<u32>
    {

        invalid_operation()
        
    }

    fn get_u64(&self) -> async_graphql::Result<u64>
    {

        invalid_operation()
        
    }

    fn get_u128(&self) -> async_graphql::Result<U128Scalar>
    {

        invalid_operation()
        
    }

    fn get_unit(&self) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn get_usize(&self) -> async_graphql::Result<usize>
    {

        invalid_operation()
        
    }

    //

    fn get_string(&self) -> async_graphql::Result<String>
    {

        invalid_operation()
        
    }

    fn get_identifier(&self) -> async_graphql::Result<Identifier>
    {

        invalid_operation()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        invalid_operation()

    }

}

//Value level atomicity storage container

pub trait StorageContainer: StorageContainerGetters + Send
{

    //non-asyc

    //ops

    fn uop(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation_instance_is_immuatable()

    }

    fn bop(&self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
        
        invalid_operation_instance_is_immuatable()

    }

    fn bop_self(&self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation_instance_is_immuatable()

    }

    //async 

    //async fn uop_async(&self, op: Uop) -> async_graphql::Result<NumericOrBool>;
    /*{
        
        //self.uop(op)

        invalid_operation_instance_is_immuatable()

    }*/

    /*
    async fn bop_async(&self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
        
        self.bop(op, right_side)

    }

    async fn bop_self_async(&self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.bop_self(op)

    }
    */

    //

    fn is_mutable(&self) -> bool
    {

        false

    }

    //

    fn set_bool(&self, input: bool) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_char(&self, input: char) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_f32(&self, input: f32) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_f64(&self, input: f64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_i8(&self, input: i8) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_i16(&self, input: i16) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_i32(&self, input: i32) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_i64(&self, input: i64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_i128(&self, input: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_isize(&self, input: isize) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    //

    fn set_u8(&self, input: u8) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_u16(&self, input: u16) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_u32(&self, input: u32) -> async_graphql::Result<UnitValue> //ctx: &Context<'_>, input: u32) -> u32
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_u64(&self, input: u64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_u128(&self, input: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_unit(&self, input: UnitValue) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_usize(&self, input: usize) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()
    }

    //Non=Primitive Types

    fn set_string(&self, input: String) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        invalid_operation_instance_is_immuatable()

    }

    fn set_identifier(&self, input: Identifier) -> async_graphql::Result<UnitValue>
    {

        invalid_operation_instance_is_immuatable()

    }

    fn set_value(&self, input: AnyInputObject) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        invalid_operation_instance_is_immuatable()

    }

}

/*
pub trait SyncStorageContainer : StorageContainer + Sync
{

}
*/

//Store and namespace level atomicity storage container

pub trait MutStorageContainer: StorageContainerGetters + Send
{

    //ops

    fn uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation()

    }

    fn bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
        
        invalid_operation()

    }

    fn bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation()

    }

    //

    fn is_mutable(&mut self) -> bool
    {

        true

    }

    //

    fn set_bool(&mut self, input: bool) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_char(&mut self, input: char) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_f32(&mut self, input: f32) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_f64(&mut self, input: f64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_i8(&mut self, input: i8) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_i16(&mut self, input: i16) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_i32(&mut self, input: i32) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_i64(&mut self, input: i64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_i128(&mut self, input: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_isize(&mut self, input: isize) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    //

    fn set_u8(&mut self, input: u8) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_u16(&mut self, input: u16) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_u32(&mut self, input: u32) -> async_graphql::Result<UnitValue> //ctx: &Context<'_>, input: u32) -> u32
    {

        invalid_operation()

    }

    fn set_u64(&mut self, input: u64) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_u128(&mut self, input: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_unit(&mut self, input: UnitValue) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_usize(&mut self, input: usize) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()
    }

    //Non=Primitive Types

    fn set_string(&mut self, input: String) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        invalid_operation()

    }

    fn set_identifier(&mut self, input: Identifier) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    fn set_value(&mut self, input: AnyInputObject) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        invalid_operation()

    }

}
