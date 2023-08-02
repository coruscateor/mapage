use std::ops::*; //{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use super::stored_unit::*;

use crate::identifier::*;

use crate::
{
    
    stored_object::StoredObject, 
    //value::*,
    all_types::*,
    errors::*,
    uniary_operations::*, async_graphql_value_containers::{NumericOrBool, NumericOrIdentifierOrBool, Numeric}, binary_operations::Bop,
    errors::*,
    //logic::Bool,
    async_graphql_value_containers::*

};

//use corlib::{HasOne, I8HasOne, I16HasOne, I32HasOne, I64HasOne, I128HasOne, ISizeHasOne, U8HasOne, U16HasOne, U32HasOne, U64HasOne, U128HasOne, USizeHasOne};

use corlib::has_one::*;

use std::marker::PhantomData;


/*
impl Default for F32HasOne
{
    fn default() -> Self {
        
        Self()

    }
}
*/

impl NumericConversions<f32> for F32HasOne
{
    fn into_numeric(value: f32) -> Numeric
    {

        Numeric::F32(F32Value { value })

    }

    fn from_numeric(value: Numeric) -> async_graphql::Result<f32> {
        
        match value
        {
            Numeric::F32(input) => Ok(input.value),
            Numeric::F64(input) => Ok(input.value as f32),
            Numeric::I8(input) => Ok(input.value as f32),
            Numeric::I16(input) => Ok(input.value as f32),
            Numeric::I32(input) => Ok(input.value as f32),
            Numeric::I64(input) => Ok(input.value as f32),
            Numeric::I128(input) => //Ok(input.value as f32),
            {

                //parse_float::<f32>(&input.value)

                Ok(input.value.0 as f32)

            }
            Numeric::Isize(input) => Ok(input.value as f32),
            Numeric::U8(input) => Ok(input.value as f32),
            Numeric::U16(input) => Ok(input.value as f32),
            Numeric::U32(input) => Ok(input.value as f32),
            Numeric::U64(input) => Ok(input.value as f32),
            Numeric::U128(input) => //Ok(input.value as f32),
            {

                //parse_float::<f32>(&input.value)

                Ok(input.value.0 as f32)

            }
            Numeric::Usize(input) => Ok(input.value as f32)
        }

    }

}

impl NumericOrBoolConversions<f32> for F32HasOne
{

    fn into_numeric_or_bool(value: f32) -> NumericOrBool {
        
        NumericOrBool::Numeric(F32HasOne::into_numeric(value))

    }

    fn from_numeric_or_bool(value: NumericOrBool) -> async_graphql::Result<f32>
    {
        
        match value {
            NumericOrBool::Numeric(input) => F32HasOne::from_numeric(input),
            NumericOrBool::Bool(_) => invalid_operation(),
        }
        
    }

}

/*
impl Default for F64HasOne
{
    fn default() -> Self {
        
        Self()

    }
}
*/

impl NumericConversions<f64> for F64HasOne
{

    fn into_numeric(value: f64) -> Numeric {
     
        Numeric::F64(F64Value { value })
        
    }

    fn from_numeric(value: Numeric) -> async_graphql::Result<f64> {
        
        match value
        {
            Numeric::F32(input) => Ok(input.value as f64),
            Numeric::F64(input) => Ok(input.value),
            Numeric::I8(input) => Ok(input.value as f64),
            Numeric::I16(input) => Ok(input.value as f64),
            Numeric::I32(input) => Ok(input.value as f64),
            Numeric::I64(input) => Ok(input.value as f64),
            Numeric::I128(input) =>
            {

                //parse_float::<f64>(&input.value)

                 //i128::
                 
                Ok(input.value.0 as f64)

            }
            Numeric::Isize(input) => Ok(input.value as f64),
            Numeric::U8(input) => Ok(input.value as f64),
            Numeric::U16(input) => Ok(input.value as f64),
            Numeric::U32(input) => Ok(input.value as f64),
            Numeric::U64(input) => Ok(input.value as f64),
            Numeric::U128(input) => //input.value as f64,
            {
                
                //parse_float::<f64>(&input.value)

                Ok(input.value.0 as f64)

            }
            Numeric::Usize(input) => Ok(input.value as f64),
        }

    }
}

impl NumericOrBoolConversions<f64> for F64HasOne
{

    fn into_numeric_or_bool(value: f64) -> NumericOrBool {
        
        NumericOrBool::Numeric(F64HasOne::into_numeric(value))

    }

    fn from_numeric_or_bool(value: NumericOrBool) -> async_graphql::Result<f64>
    {

        match value {
            NumericOrBool::Numeric(input) => F64HasOne::from_numeric(input),
            NumericOrBool::Bool(_) => invalid_operation(),
        }
        
    }

}

//struct IntoNumeric

#[derive(Default)]
pub struct StoredNumeric<T, HO> 
{

    value: T,
    pt: PhantomData<HO>

}

impl<T, HO> StoredNumeric<T, HO>
    where T: ProbablyNumeric + std::ops::Neg<Output = T> + Not + std::ops::Not<Output = T>,
          HO: HasOne<T> + NumericOrBoolConversions<T>
{
    
    pub fn new(value: T) -> Self
    {

        Self
        {

            value,
            pt: PhantomData::default()

        }

    }

    fn uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        match op
        {

            Uop::Inc =>
            {

                self.value += HO::one();

                //return Ok(HO::into_numeric_or_bool(self.value));

            },
            Uop::Dec => 
            {

                self.value -= HO::one();

                //return Ok(HO::into_numeric_or_bool(self.value));

            },
            Uop::Neg =>
            {

                self.value = -self.value;

                //return Ok(HO::into_numeric_or_bool(self.value));

            }
            Uop::Not =>
            {

                self.value = !self.value;

                //return Ok(HO::into_numeric_or_bool(self.value));

            }
        }

        Ok(HO::into_numeric_or_bool(self.value))

        //invalid_operation()

    }

    fn bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> //NumericOrKeyNSOrBool
    {

        let res = HO::from_numeric_or_bool(right_side);

        match res
        {

            Ok(val) =>
            {

                match op
                {

                    Bop::Add => self.value += val,
                    Bop::BitAnd => self.value &= val,
                    Bop::BitOr => self.value |= val,
                    Bop::BitXor => self.value ^= val,
                    Bop::Div => self.value /= val,
                    Bop::Mul => self.value *= val,
                    Bop::Rem => self.value %= val,
                    Bop::Shl => self.value <<= val,
                    Bop::Shr => self.value >>= val,
                    Bop::Sub => self.value -= val,

                }

            }
            Err(err) => return Err(err),

        }

        Ok(HO::into_numeric_or_bool(self.value))

    }

    fn bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        match op
        {

            Bop::Add => self.value += self.value,
            Bop::BitAnd => self.value &= self.value,
            Bop::BitOr => self.value |= self.value,
            Bop::BitXor => self.value ^= self.value,
            Bop::Div => self.value /= self.value,
            Bop::Mul => self.value *= self.value,
            Bop::Rem => self.value %= self.value,
            Bop::Shl => self.value <<= self.value,
            Bop::Shr => self.value >>= self.value,
            Bop::Sub => self.value -= self.value,

        }

        Ok(HO::into_numeric_or_bool(self.value))

    }

}

/*
impl<T> StoredObject for StoredNumeric<T>
    where T: Send
{

    fn get_type(&self, _all_types: &AllTypes) -> Type
    {

        Type::Nothing //Add error type?

    }

}
*/

//Floating point values

impl StoredObject for StoredNumeric<f32, F32HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::f32

        AllTypes::get_f32_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::F32(F32Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

    //method forwarding macro

}

impl StoredObject for StoredNumeric<f64, F64HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::f64

        AllTypes::get_f64_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::F64(F64Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

}

//Signed integers

impl StoredObject for StoredNumeric<i8, I8HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::i8

        AllTypes::get_i8_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::I8(I8Value::new(self.value)))))

    }

    /* 
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

}

impl StoredObject for StoredNumeric<i16, I16HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::i16

        AllTypes::get_i16_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::I16(I16Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

}

impl StoredObject for StoredNumeric<i32, I32HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::i32

        AllTypes::get_i32_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::I32(I32Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

}

impl StoredObject for StoredNumeric<i64, I64HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::i64

        AllTypes::get_i64_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::I64(I64Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */


}

impl StoredObject for StoredNumeric<i128, I128HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::i128

        AllTypes::get_i128_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::I128(I128Value::new(self.value)))))

    }

    /*
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        Ok(self.value as bool)
        
    }
    */

}

impl StoredObject for StoredNumeric<isize, ISizeHasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::isize

        AllTypes::get_isize_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::Isize(ISizeValue::new(self.value)))))

    }

}

//unsigned integers

impl StoredObject for StoredNumeric<u8, U8HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::u8

        AllTypes::get_u8_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::U8(U8Value::new(self.value)))))

    }

}

impl StoredObject for StoredNumeric<u16, U16HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::u16

        AllTypes::get_u16_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::U16(U16Value::new(self.value)))))

    }

}

impl StoredObject for StoredNumeric<u32, U32HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::u32

        AllTypes::get_u32_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::U32(U32Value::new(self.value)))))

    }

}

impl StoredObject for StoredNumeric<u64, U64HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::u64

        AllTypes::get_u64_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::U64(U64Value::new(self.value)))))

    }

}

impl StoredObject for StoredNumeric<u128, U128HasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::u128

        AllTypes::get_u128_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::U128(U128Value::new(self.value)))))

    }

}

impl StoredObject for StoredNumeric<usize, USizeHasOne>
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {

        //Type::usize

        AllTypes::get_usize_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        Ok(Some(AnyObject::Numeric(Numeric::Usize(USizeValue::new(self.value)))))

    }

}