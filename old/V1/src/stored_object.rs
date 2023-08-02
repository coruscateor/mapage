//use crate::memrs_type::*; //Type;
use std::ops::*;
use crate::consts::UnitValue;
//use async_graphql::{Result, Error};
use crate::errors::invalid_operation; 

//pub struct Object;

//use crate::non_generic_type_controller::*;

use crate::errors::invalid_operation_instance_is_immuatable;
use crate::identifier::*;
use crate::uniary_operations::*;

use crate::binary_operations::*;

use crate::ternary_operations::*;

use crate::async_graphql_value_containers::*;

use crate::all_types::*;

//#[async_graphql::Object]
#[allow(unused_variables)]
pub trait StoredObject : Send //+ Sync //: Add
{

    fn get_type(&self) -> Type; //, all_types: &AllTypes

    //, identifier: KeyNS

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

    //fn get_type_ref(&self) -> &Type;

    /*
    fn increment(&mut self) -> async_graphql::Result<()>
    {

        invalid_operation()

    }

    fn decrement(&mut self) -> async_graphql::Result<()>
    {

        invalid_operation()

    }
    */

    //numeric ops

    //fn add_f32(&mut self, other: f32) -> Result<f32>;

    /*
    fn add_f32(&mut self, other: f32) -> async_graphql::Result<f32>
    {
        
        invalid_operation()

    }
    */

    //fn uop(&self, op: Uop) -> async_graphql::Result<NumericOrBool>; //uniary operation //UniaryOp //, right_side: NumericOrKeyNSOrBool

    //fn uop_mut(&self, op: Uop) -> async_graphql::Result<NumericOrBool>;

    //fn bop(&self, op: Bop, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>; //binary operation //BinaryOp

    //fn bop_mut(&self, op: Bop, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>;

    //fn top(&self, op: Top, middle: NumericOrKeyNSOrBool, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>; //ternary operation //TernaryOp

    //fn top_mut(&self, op: Top, middle: NumericOrKeyNSOrBool, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>;

    fn uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        //invalid_operation_instance_is_immuatable()

        invalid_operation()

    }

    fn bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> //NumericOrKeyNSOrBool
    {
        
        //invalid_operation_instance_is_immuatable()

        invalid_operation()

    }

    fn bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation()

    }

    //fn is_mutable(&self) -> bool;

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

        //Ok(I128::default())

    }

    fn set_isize(&mut self, input: isize) -> async_graphql::Result<UnitValue>
    {

        invalid_operation()

    }

    //pointer?

    //reference?

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

    //string value
    fn set_string(&mut self, nput: String) -> async_graphql::Result<UnitValue> //, input: String
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