use std::collections::HashMap;

use crate::async_graphql_types::{NonGenericType, GenericType};
use crate::async_graphql_value_containers::{NumericOrBool, AnyObject};
use crate::binary_operations::Bop;
use crate::consts::UnitValue;
use crate::identifier::*;
use crate::immut_mut_value_storage_container::*;
use crate::mut_value_storage_container::*;
use crate::storage_container::MutStorageContainer;
use crate::uniary_operations::Uop;
use crate::{stored_object::*, storage_container::StorageContainer};
use crate::errors::*;

use crate::{

    refcell_storage_container::*,
    namespace_getters::*,
    namespace_setters_mut::*,
    async_graphql_value_containers::*

};

use crate::actor_utils::*;

use crate::stored_objects::*;

use corlib::has_one::*;

use stored_bool::*;

use stored_char::*;

use stored_numeric::*;

use stored_unit::*;

use std_string::*;

use crate::all_types::*;

//use std::collections::HashMap;

pub struct Namespace
{

    kvs: HashMap<String, Box<dyn MutStorageContainer>>

}

impl Namespace
{

    pub fn new() -> Self
    {

        Self
        {

            kvs: HashMap::new()

        }

    }

    fn try_insert_kvp<T>(&mut self, identifier: &Identifier, bst: T) -> async_graphql::Result<UnitValue>
        where T: MutStorageContainer + 'static
    {

        if self.kvs.contains_key(identifier.get_name_ref())
        {

            stored_object_already_exists_in_specified_namespace()

        }
        else
        {
            
            self.kvs.insert(identifier.get_name_ref().clone(), Box::new(bst));

            Ok(UnitValue::new())

        }
    

    }

    //create string (with\without params)

    //

    fn kvs_get<T, F>(&self, identifier: &Identifier, f: F) -> async_graphql::Result<T>
        where F: Fn(&Box<dyn MutStorageContainer>) -> async_graphql::Result<T>
    {

        if let Some(val) = self.kvs.get(identifier.get_name_ref())
        {

            f(val)

        }
        else
        {
        
            storage_not_found_error()
            
        }

    }

    fn kvs_get_mut<T, F>(&mut self, identifier: &Identifier, f: F) -> async_graphql::Result<T>
        where F: Fn(&mut Box<dyn MutStorageContainer>) -> async_graphql::Result<T>
    {

        if let Some(val) = self.kvs.get_mut(identifier.get_name_ref())
        {

            f(val)

        }
        else
        {
        
            storage_not_found_error()
            
        }

    }

    fn kvs_set<T, F, V>(&mut self, identifier: &Identifier, value: V, f: F) -> async_graphql::Result<T>
        where F: Fn(&mut Box<dyn MutStorageContainer>, V) -> async_graphql::Result<T>
    {

        if let Some(val) = self.kvs.get_mut(identifier.get_name_ref())
        {

            f(val, value)

        }
        else
        {
        
            storage_not_found_error()
            
        }

    }

    
}


impl NamespaceGetters for Namespace
{

    fn get_bool(&self, identifier: &Identifier) -> async_graphql::Result<bool>
    {

        self.kvs_get(identifier, |val| val.get_bool() )
        
    }

    fn get_char(&self, identifier: &Identifier) -> async_graphql::Result<char>
    {

        self.kvs_get(identifier, |val| val.get_char() )
        
    }

    fn get_f32(&self, identifier: &Identifier) -> async_graphql::Result<f32>
    {

        self.kvs_get(identifier, |val| val.get_f32() )
        
    }

    fn get_f64(&self, identifier: &Identifier) -> async_graphql::Result<f64>
    {

        self.kvs_get(identifier, |val| val.get_f64() )
        
    }

    fn get_i8(&self, identifier: &Identifier) -> async_graphql::Result<i8>
    {

        self.kvs_get(identifier, |val| val.get_i8() )
        
    }

    fn get_i16(&self, identifier: &Identifier) -> async_graphql::Result<i16>
    {

        self.kvs_get(identifier, |val| val.get_i16() )
        
    }

    fn get_i32(&self, identifier: &Identifier) -> async_graphql::Result<i32>
    {

        self.kvs_get(identifier, |val| val.get_i32() )
        
    }

    fn get_i64(&self, identifier: &Identifier) -> async_graphql::Result<i64>
    {

        self.kvs_get(identifier, |val| val.get_i64() )
        
    }

    fn get_i128(&self, identifier: &Identifier) -> async_graphql::Result<I128Scalar>
    {

        self.kvs_get(identifier, |val| val.get_i128() )
        
    }

    fn get_isize(&self, identifier: &Identifier) -> async_graphql::Result<isize>
    {

        self.kvs_get(identifier, |val| val.get_isize() )
        
    }

    //

    fn get_u8(&self, identifier: &Identifier) -> async_graphql::Result<u8>
    {

        self.kvs_get(identifier, |val| val.get_u8() )
        
    }

    fn get_u16(&self, identifier: &Identifier) -> async_graphql::Result<u16>
    {

        self.kvs_get(identifier, |val| val.get_u16() )
        
    }

    fn get_u32(&self, identifier: &Identifier) -> async_graphql::Result<u32>
    {

        self.kvs_get(identifier, |val| val.get_u32() )
        
    }

    fn get_u64(&self, identifier: &Identifier) -> async_graphql::Result<u64>
    {

        self.kvs_get(identifier, |val| val.get_u64() )
        
    }

    fn get_u128(&self, identifier: &Identifier) -> async_graphql::Result<U128Scalar>
    {

        self.kvs_get(identifier, |val| val.get_u128() )
        
    }

    fn get_unit(&self, identifier: &Identifier) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get(identifier, |val| val.get_unit() )

    }

    fn get_usize(&self, identifier: &Identifier) -> async_graphql::Result<usize>
    {

        self.kvs_get(identifier, |val| val.get_usize() )
        
    }

    //

    fn get_string(&self, identifier: &Identifier) -> async_graphql::Result<String>
    {

        self.kvs_get(identifier, |val| val.get_string() )
        
    }

    fn get_identifier(&self, identifier: &Identifier) -> async_graphql::Result<Identifier>
    {

        self.kvs_get(identifier, |val| val.get_identifier() )

    }

    fn get_value(&self, identifier: &Identifier) -> async_graphql::Result<Option<AnyObject>>
    {

        self.kvs_get(identifier, |val| val.get_value() )

    }

    fn get_count(&self) -> usize
    {

        self.kvs.len()

    }

    fn get_value_names(&self) -> Vec<String> {
        
        let mut val_names = Vec::with_capacity(self.kvs.len());

        for item in self.kvs.iter()
        {

            val_names.push(item.0.clone());

        }

        val_names

    }

    fn get_value_names_and_types(&self) -> std::collections::HashMap<String, Type> {
       
        let mut val_names_and_types = std::collections::HashMap::with_capacity(self.kvs.len());

        for item in self.kvs.iter()
        {

            //val_names_and_types.push((item.0.clone(), item.1.get_type()));

            val_names_and_types.insert(item.0.clone(), item.1.get_type());

        }

        val_names_and_types
        
    }

    fn get_type(&self, identifier: &Identifier) -> async_graphql::Result<Type>
    {
       
        self.kvs_get(identifier, |val| Ok(val.get_type()) )

    }


}

impl NamespaceSettersMut for Namespace
{

    fn create_stored_object(&mut self, identifier: &Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        type Cont<T> = ImmutMutValueStorageContainer<T>;

        match the_type
        {
            NonGenericType::Bool => return self.try_insert_kvp(identifier, Cont::new(StoredBool::default())),
            NonGenericType::Char => return self.try_insert_kvp(identifier, Cont::new(StoredChar::default())),
            NonGenericType::F32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<f32, F32HasOne>::default())),
            NonGenericType::F64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<f64, F64HasOne>::default())),
            NonGenericType::I8 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i8, I8HasOne>::default())),
            NonGenericType::I16 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i16, I16HasOne>::default())),
            NonGenericType::I32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i32, I32HasOne>::default())),
            NonGenericType::I64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i64, I64HasOne>::default())),
            NonGenericType::I128 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i128, I128HasOne>::default())),
            NonGenericType::Isize => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<isize, ISizeHasOne>::default())),
            NonGenericType::U8 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u8, U8HasOne>::default())),
            NonGenericType::U16 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u16, U16HasOne>::default())),
            NonGenericType::U32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u32, U32HasOne>::default())),
            NonGenericType::U64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u64, U64HasOne>::default())),
            NonGenericType::U128 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u128, U128HasOne>::default())),
            NonGenericType::Unit => return self.try_insert_kvp(identifier, Cont::new(StoredUnit::default())),
            NonGenericType::Usize => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<usize, USizeHasOne>::default())),
            NonGenericType::String => return self.try_insert_kvp(identifier, Cont::new(StdString::default())),
            
        }

        invalid_operation()

    }

    fn create_stored_object_mut(&mut self, identifier: &Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>
    {

        type Cont<T> = MutValueStorageContainer<T>;

        match the_type
        {
            
            NonGenericType::Bool => return self.try_insert_kvp(identifier, Cont::new(StoredBool::default())),
            NonGenericType::Char => return self.try_insert_kvp(identifier, Cont::new(StoredChar::default())),
            NonGenericType::F32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<f32, F32HasOne>::default())),
            NonGenericType::F64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<f64, F64HasOne>::default())),
            NonGenericType::I8 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i8, I8HasOne>::default())),
            NonGenericType::I16 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i16, I16HasOne>::default())),
            NonGenericType::I32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i32, I32HasOne>::default())),
            NonGenericType::I64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i64, I64HasOne>::default())),
            NonGenericType::I128 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<i128, I128HasOne>::default())),
            NonGenericType::Isize => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<isize, ISizeHasOne>::default())),
            NonGenericType::U8 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u8, U8HasOne>::default())),
            NonGenericType::U16 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u16, U16HasOne>::default())),
            NonGenericType::U32 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u32, U32HasOne>::default())),
            NonGenericType::U64 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u64, U64HasOne>::default())),
            NonGenericType::U128 => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<u128, U128HasOne>::default())),
            NonGenericType::Unit => return self.try_insert_kvp(identifier, Cont::new(StoredUnit::default())),
            NonGenericType::Usize => return self.try_insert_kvp(identifier, Cont::new(StoredNumeric::<usize, USizeHasOne>::default())),
            NonGenericType::String => return self.try_insert_kvp(identifier, Cont::new(StdString::default())),

        }

        invalid_operation()

    }

    fn create_generic_stored_object(&mut self, identifier: &Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>
    {

        Ok(UnitValue::new())

    }

    fn create_generic_stored_object_mut(&mut self, identifier: &Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>
    {

        Ok(UnitValue::new())

    }

    fn uop(&mut self, identifier: &Identifier, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        self.kvs_get_mut(identifier, |val| val.uop(op) )


    }

    fn bop(&mut self, identifier: &Identifier, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> 
    {
       
        self.kvs_get_mut(identifier, |val| val.bop(op, right_side ) )
    

    }

    fn bop_self(&mut self, identifier: &Identifier, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.kvs_get_mut(identifier, |val| val.bop_self(op) )

    }

    fn set_bool(&mut self, identifier: &Identifier, input: bool) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_bool(input))
       
    }

    fn set_char(&mut self, identifier: &Identifier, input: char) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_char(input))
        
    }

    fn set_f32(&mut self, identifier: &Identifier, input: f32) -> async_graphql::Result<UnitValue>
    {
        
        self.kvs_get_mut(identifier, |val| val.set_f32(input))

    }

    fn set_f64(&mut self, identifier: &Identifier, input: f64) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_f64(input))

    }

    fn set_i8(&mut self, identifier: &Identifier, input: i8) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_i8(input))

    }

    fn set_i16(&mut self, identifier: &Identifier, input: i16) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_i16(input))

    }

    fn set_i32(&mut self, identifier: &Identifier, input: i32) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_i32(input))

    }

    fn set_i64(&mut self, identifier: &Identifier, input: i64) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_i64(input))

    }

    fn set_i128(&mut self, identifier: &Identifier, input: I128Scalar) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_i128(input))

    }

    fn set_isize(&mut self, identifier: &Identifier, input: isize) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_isize(input))

    }

    fn set_u8(&mut self, identifier: &Identifier, input: u8) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_u8(input))

    }

    fn set_u16(&mut self, identifier: &Identifier, input: u16) -> async_graphql::Result<UnitValue>
    {
       
        self.kvs_get_mut(identifier, |val| val.set_u16(input))

    }

    fn set_u32(&mut self, identifier: &Identifier, input: u32) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_u32(input))

    }

    fn set_u64(&mut self, identifier: &Identifier, input: u64) -> async_graphql::Result<UnitValue>
    {
        
        self.kvs_get_mut(identifier, |val| val.set_u64(input))
        
    }

    fn set_u128(&mut self, identifier: &Identifier, input: U128Scalar) -> async_graphql::Result<UnitValue>
    {
        
        self.kvs_get_mut(identifier, |val| val.set_u128(input))

    }

    fn set_unit(&mut self, identifier: &Identifier, input: UnitValue) -> async_graphql::Result<UnitValue>
    {

        self.kvs_get_mut(identifier, |val| val.set_unit(input) )

    }

    fn set_usize(&mut self, identifier: &Identifier, input: usize) -> async_graphql::Result<UnitValue>
    {
        
        self.kvs_get_mut(identifier, |val| val.set_usize(input))

    }

    fn set_string(&mut self, identifier: &Identifier, input: String) -> async_graphql::Result<UnitValue>
    {

        self.kvs_set(identifier, input, |val, input| val.set_string(input))

    }

    fn set_identifier(&mut self, identifier: &Identifier, input: Identifier) -> async_graphql::Result<UnitValue>
    {

        self.kvs_set(identifier, input, |val, input| val.set_identifier(input))

    }

    fn set_value(&mut self, identifier: &Identifier, input: AnyInputObject) -> async_graphql::Result<UnitValue>
    {

        self.kvs_set(identifier, input, |val, input| val.set_value(input))

    }

    /*
    fn is_mutable(&mut self) -> bool
    {

        true

    }
    */

}