use crate::{stored_object::StoredObject, storage_container::{StorageContainer, StorageContainerGetters, MutStorageContainer}, all_types::{AllTypes, Type}};

use crate::async_graphql_value_containers::*;

use crate::identifier::*;

use crate::errors::*;

//sdo: stored object

///A container that guarantees a stored objects immutability
/// 
/// Store and namespace level immutabity
/// 
/// MutValueStorageContainer<T> is a mutable counterpart struct
pub struct ImmutMutValueStorageContainer<T>
    where T: StoredObject
{

    sdo: T

}

impl<T> ImmutMutValueStorageContainer<T>
    where T: StoredObject
{

    pub fn new(sdo: T) -> Self
    {

        Self
        {

            sdo

        }

    }

}

impl<T> StorageContainerGetters for ImmutMutValueStorageContainer<T>
    where T: StoredObject + Sync
{

    fn get_type(&self) -> Type //, all_types: &AllTypes
    {
        
        self.sdo.get_type() //all_types)

    }

    fn get_bool(&self) -> async_graphql::Result<bool> 
    {
        
        self.sdo.get_bool()
        
    }

    fn get_char(&self) -> async_graphql::Result<char>
    {

        self.sdo.get_char()
        
    }

    fn get_f32(&self) -> async_graphql::Result<f32>
    {

        self.sdo.get_f32()
        
    }

    fn get_f64(&self) -> async_graphql::Result<f64>
    {

        self.sdo.get_f64()
        
    }

    fn get_i8(&self) -> async_graphql::Result<i8>
    {

        self.sdo.get_i8()
        
    }

    fn get_i16(&self) -> async_graphql::Result<i16>
    {

        self.sdo.get_i16()
        
    }

    fn get_i32(&self) -> async_graphql::Result<i32>
    {

        self.sdo.get_i32()
        
    }

    fn get_i64(&self) -> async_graphql::Result<i64>
    {

        self.sdo.get_i64()
        
    }

    fn get_i128(&self) -> async_graphql::Result<I128Scalar>
    {

        self.sdo.get_i128()
        
    }

    fn get_isize(&self) -> async_graphql::Result<isize>
    {

        self.sdo.get_isize()
        
    }

    //

    fn get_u8(&self) -> async_graphql::Result<u8>
    {

        self.sdo.get_u8()
        
    }

    fn get_u16(&self) -> async_graphql::Result<u16>
    {

        self.sdo.get_u16()
        
    }

    fn get_u32(&self) -> async_graphql::Result<u32>
    {

        self.sdo.get_u32()
        
    }

    fn get_u64(&self) -> async_graphql::Result<u64>
    {

        self.sdo.get_u64()
        
    }

    fn get_u128(&self) -> async_graphql::Result<U128Scalar>
    {

        self.sdo.get_u128()
        
    }

    fn get_usize(&self) -> async_graphql::Result<usize>
    {

        self.sdo.get_usize()
        
    }

    //

    fn get_string(&self) -> async_graphql::Result<String>
    {

        self.sdo.get_string()
        
    }

    fn get_identifier(&self) -> async_graphql::Result<Identifier>
    {

        self.sdo.get_identifier()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        self.sdo.get_value()

    }

}

impl<T> MutStorageContainer for ImmutMutValueStorageContainer<T>
    where T: StoredObject + Sync
{

    fn uop(&mut self, op: crate::uniary_operations::Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn bop(&mut self, op: crate::binary_operations::Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
        
        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn bop_self(&mut self, op: crate::binary_operations::Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn is_mutable(&mut self) -> bool
    {

        false

    }

    fn set_bool(&mut self, input: bool) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_char(&mut self, input: char) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_f32(&mut self, input: f32) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_f64(&mut self, input: f64) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_i8(&mut self, input: i8) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_i16(&mut self, input: i16) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_i32(&mut self, input: i32) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_i64(&mut self, input: i64) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_i128(&mut self, input: I128Scalar) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_isize(&mut self, input: isize) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_u8(&mut self, input: u8) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_u16(&mut self, input: u16) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_u32(&mut self, input: u32) -> async_graphql::Result<crate::consts::UnitValue> {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_u64(&mut self, input: u64) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_u128(&mut self, input: U128Scalar) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_usize(&mut self, input: usize) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()
    }

    fn set_string(&mut self, input: String) -> async_graphql::Result<crate::consts::UnitValue> {

        //May have to clone it

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_identifier(&mut self, input: Identifier) -> async_graphql::Result<crate::consts::UnitValue>
    {

        crate::errors::invalid_operation_instance_is_immuatable()

    }

    fn set_value(&mut self, input: AnyInputObject) -> async_graphql::Result<crate::consts::UnitValue> {

        //May have to clone it

        crate::errors::invalid_operation_instance_is_immuatable()

    }

}

