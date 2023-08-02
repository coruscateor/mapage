use crate::{stored_object::StoredObject, storage_container::{StorageContainer, StorageContainerGetters}, all_types::{AllTypes, Type}, async_graphql_value_containers::{NumericOrBool, AnyObject, U128Scalar, I128Scalar, AnyInputObject}, uniary_operations::Uop, binary_operations::Bop, identifier::*, consts::UnitValue};

use std::{cell::RefCell, borrow::BorrowMut};

//sdo: stored object

///A container for stored objects that are expected to be mutable
///
///Suitable for Stores that have Store-level or Namespace-level atomicity (can't be used on situations where multiple threads can access each value of the store individually)  
/// 
/// ImmutValueStorageContainer<T> is the immutable counterpart struct
pub struct RefCellStorageContainer<T>
    where T: StoredObject
{

    sdo: RefCell<T>

}

impl<T> RefCellStorageContainer<T>
    where T: StoredObject
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            sdo: RefCell::new(object)

        }

    }

}

impl<T> StorageContainerGetters for RefCellStorageContainer<T>
    where T: StoredObject + Sync
{

    fn get_type(&self) -> Type //, all_types: &AllTypes
    {
        
        
        self.sdo.borrow().get_type() //all_types)

    }

    fn get_bool(&self) -> async_graphql::Result<bool>
    {
        
        self.sdo.borrow().get_bool()
        
    }

    fn get_char(&self) -> async_graphql::Result<char>
    {

        self.sdo.borrow().get_char()
        
    }

    fn get_f32(&self) -> async_graphql::Result<f32>
    {

        self.sdo.borrow().get_f32()
        
    }

    fn get_f64(&self) -> async_graphql::Result<f64>
    {

        self.sdo.borrow().get_f64()
        
    }

    fn get_i8(&self) -> async_graphql::Result<i8>
    {

        self.sdo.borrow().get_i8()
        
    }

    fn get_i16(&self) -> async_graphql::Result<i16>
    {

        self.sdo.borrow().get_i16()
        
    }

    fn get_i32(&self) -> async_graphql::Result<i32>
    {

        self.sdo.borrow().get_i32()
        
    }

    fn get_i64(&self) -> async_graphql::Result<i64>
    {

        self.sdo.borrow().get_i64()
        
    }

    fn get_i128(&self) -> async_graphql::Result<I128Scalar>
    {

        self.sdo.borrow().get_i128()
        
    }

    fn get_isize(&self) -> async_graphql::Result<isize>
    {

        self.sdo.borrow().get_isize()
        
    }

    //

    fn get_u8(&self) -> async_graphql::Result<u8>
    {

        self.sdo.borrow().get_u8()
        
    }

    fn get_u16(&self) -> async_graphql::Result<u16>
    {

        self.sdo.borrow().get_u16()
        
    }

    fn get_u32(&self) -> async_graphql::Result<u32>
    {

        self.sdo.borrow().get_u32()
        
    }

    fn get_u64(&self) -> async_graphql::Result<u64>
    {

        self.sdo.borrow().get_u64()
        
    }

    fn get_u128(&self) -> async_graphql::Result<U128Scalar>
    {

        self.sdo.borrow().get_u128()
        
    }

    fn get_usize(&self) -> async_graphql::Result<usize>
    {

        self.sdo.borrow().get_usize()
        
    }

    //

    fn get_string(&self) -> async_graphql::Result<String>
    {

        self.sdo.borrow().get_string()
        
    }

    fn get_identifier(&self) -> async_graphql::Result<Identifier>
    {

        self.sdo.borrow().get_identifier()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>> //, identifier: Identifier
    {

        self.sdo.borrow().get_value()

    }

}

impl<T> StorageContainer for RefCellStorageContainer<T>
    where T: StoredObject + Sync
{

    fn uop(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.sdo.borrow_mut().uop(op)

    }

    fn bop(&self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
        
        self.sdo.borrow_mut().bop(op, right_side)

    }

    fn bop_self(&self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.sdo.borrow_mut().bop_self(op)

    }

    fn is_mutable(&self) -> bool
    {

        true

    }

    fn set_bool(&self, input: bool) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_bool(input)

    }

    fn set_char(&self, input: char) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_char(input)

    }

    fn set_f32(&self, input: f32) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_f32(input)

    }

    fn set_f64(&self, input: f64) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_f64(input)

    }

    fn set_i8(&self, input: i8) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_i8(input)

    }

    fn set_i16(&self, input: i16) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_i16(input)

    }

    fn set_i32(&self, input: i32) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_i32(input)

    }

    fn set_i64(&self, input: i64) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_i64(input)

    }

    fn set_i128(&self, input: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_i128(input)

    }

    fn set_isize(&self, input: isize) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_isize(input)

    }

    //

    fn set_u8(&self, input: u8) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_u8(input)

    }

    fn set_u16(&self, input: u16) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_u16(input)

    }

    fn set_u32(&self, input: u32) -> async_graphql::Result<UnitValue> //ctx: &Context<'_>, input: u32) -> u32
    {

        self.sdo.borrow_mut().set_u32(input)

    }

    fn set_u64(&self, input: u64) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_u64(input)

    }

    fn set_u128(&self, input: U128Scalar) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_u128(input)

    }

    fn set_usize(&self, input: usize) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_usize(input)

    }

    //Non=Primitive Types

    fn set_string(&self, input: String) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        self.sdo.borrow_mut().set_string(input)

    }

    fn set_identifier(&self, input: Identifier) -> async_graphql::Result<UnitValue>
    {

        self.sdo.borrow_mut().set_identifier(input)

    }

    fn set_value(&self, input: AnyInputObject) -> async_graphql::Result<UnitValue> //, input: String
    {

        //May have to clone it

        self.sdo.borrow_mut().set_value(input)

    }


}