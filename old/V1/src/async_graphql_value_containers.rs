use std::ops::*;

use async_graphql::*;
use serde::{Serialize, Deserialize};
use tokio::io::copy;

use crate::{identifier::*, consts::*};

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "BoolValueInput")]
pub struct BoolValue
{

    pub value: bool

}

impl BoolValue
{
    
    pub fn new(value: bool) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "CharValueInput")]
pub struct CharValue
{

    pub value: char

}

impl CharValue
{
    
    pub fn new(value: char) -> Self
    {

        Self
        {

            value

        }

    }

}

//numeric

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "F32ValueInput")]
pub struct F32Value
{

    pub value: f32

}

impl F32Value
{
    
    pub fn new(value: f32) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "F64ValueInput")]
pub struct F64Value
{

    pub value: f64

}

impl F64Value
{
    
    pub fn new(value: f64) -> Self
    {

        Self
        {

            value

        }

    }

}

//Signed Integers

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "I8ValueInput")]
pub struct I8Value
{

    pub value: i8

}

impl I8Value
{
    
    pub fn new(value: i8) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "I16ValueInput")]
pub struct I16Value
{

    pub value: i16

}

impl I16Value
{
    
    pub fn new(value: i16) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "I32ValueInput")]
pub struct I32Value
{

    pub value: i32

}

impl I32Value
{
    
    pub fn new(value: i32) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "I64ValueInput")]
pub struct I64Value
{

    pub value: i64

}

impl I64Value
{
    
    pub fn new(value: i64) -> Self
    {

        Self
        {

            value

        }

    }

}

//https://async-graphql.github.io/async-graphql/en/custom_scalars.html

//Custom Scalar

#[derive(Serialize, Deserialize, Default, Clone, Copy)] //InputObject, 
pub struct I128Scalar(pub i128);

scalar!(I128Scalar);

/*
#[Scalar]
impl ScalarType for I128Scalar
{
    fn parse(value: Value) -> InputValueResult<Self> {
        
        if let Value::String(value) = &value {
            // Parse the integer value
            Ok(value.parse().map(I128Scalar)?)
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
        
    }

    fn to_value(&self) -> Value {

        Value::String(self.0.to_string())

    }

}
*/

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "I28ValueInput")]
pub struct I128Value
{

    //pub value: i128

    //pub value: String

    pub value: I128Scalar

}

impl I128Value
{
    
    pub fn new(value: i128) -> Self
    {

        Self
        {

            //value: value.to_string()

            value: I128Scalar(value)

        }

    }

}

/*
impl Default for I128
{

    fn default() -> Self
    {

        Self
        {
            
            value: i128::default().to_string()
        
        }

    }

}
*/

//

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "ISizeValueInput")]
pub struct ISizeValue
{

    pub value: isize

}

impl ISizeValue
{

    pub fn new(value: isize) -> Self
    {

        Self
        {

            value

        }

    }

}

//Unsigned Integers

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "U8ValueInput")]
pub struct U8Value
{

    pub value: u8

}

impl U8Value
{

    pub fn new(value: u8) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "U16ValueInput")]
pub struct U16Value
{

    pub value: u16

}

impl U16Value
{

    pub fn new(value: u16) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "U32ValueInput")]
pub struct U32Value
{

    pub value: u32

}

impl U32Value
{

    pub fn new(value: u32) -> Self
    {

        Self
        {

            value

        }

    }

}

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "U64ValueInput")]
pub struct U64Value
{

    pub value: u64

}

impl U64Value
{

    pub fn new(value: u64) -> Self
    {

        Self
        {

            value

        }

    }

}


//Custom Scalar

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct U128Scalar(pub u128);

scalar!(U128Scalar);

/*
#[Scalar]
impl ScalarType for U128Scalar
{
    
    fn parse(value: Value) -> InputValueResult<Self>
    {
        
        if let Value::String(value) = &value {
            // Parse the integer value
            Ok(value.parse().map(U128Scalar)?)
        }
        else
        {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
        
    }

    fn to_value(&self) -> Value {

        Value::String(self.0.to_string())

    }
    
}
*/


#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "U128ValueInput")]
pub struct U128Value
{

    //pub value: i128

    //pub value: String

    pub value: U128Scalar

}

impl U128Value
{
    
    pub fn new(value: u128) -> Self
    {

        Self
        {

            value: U128Scalar(value) //value.to_string()

        }

    }

}

/*
impl Default for U128
{

    fn default() -> Self
    {

        Self
        {
            
            value: u128::default().to_string()
        
        }

    }

}
*/

//

#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[graphql(input_name = "USizeValueInput")]
pub struct USizeValue
{

    pub value: usize

}

impl USizeValue
{

    pub fn new(value: usize) -> Self
    {

        Self
        {

            value

        }

    }

}


#[derive(Union, Clone, Copy)]
pub enum Numeric
{

    
    F32(F32Value),
    F64(F64Value),
    I8(I8Value),
    I16(I16Value),
    I32(I32Value),
    I64(I64Value),

    I128(I128Value),
    Isize(ISizeValue),
    U8(U8Value),
    U16(U16Value),
    U32(U32Value),
    U64(U64Value),

    U128(U128Value),
    Usize(USizeValue),

    
}

#[derive(Union)] //Copy, Clone)
pub enum NumericOrIdentifier
{

    #[graphql(flatten)]
    Numeric(Numeric),
    Identifier(Identifier)

}

#[derive(Union, Clone, Copy)]
pub enum NumericOrBool
{

    #[graphql(flatten)]
    Numeric(Numeric),
    Bool(BoolValue)

}

//

#[derive(InputObject, SimpleObject, Default, Clone)]
#[graphql(input_name = "StringValueInput")]
pub struct StringValue
{

    pub value: String

}

impl StringValue
{
    
    pub fn new(value: String) -> Self
    {

        Self
        {

            value

        }

    }

}

//

#[derive(Union)]
pub enum NumericBoolOrString
{

    #[graphql(flatten)]
    Numeric(NumericOrBool),
    String(StringValue)

}

#[derive(Union)]
pub enum NumericOrIdentifierOrBool
{

    #[graphql(flatten)]
    Numeric(Numeric),
    Identifier(Identifier),
    Bool(BoolValue)

}

pub trait ProbablyNumeric : Send + Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Sized + Copy + PartialEq + Eq + Default
{

}

pub trait NumericConversions<T>
    //where T: ProbablyNumeric
{

    fn from_numeric(value: Numeric) -> async_graphql::Result<T>;

    fn into_numeric(value: T) -> Numeric;

}

pub trait NumericOrBoolConversions<T> : NumericConversions<T>
    //where T: ProbablyNumeric
{

    fn from_numeric_or_bool(value: NumericOrBool) -> async_graphql::Result<T>;

    fn into_numeric_or_bool(value: T) -> NumericOrBool;

}

//

//https://github.com/djkoloski/rust_serialization_benchmark

//https://serde.rs/

///
/// Serialisation results storage for stored objects, particularly collections.
/// Binary formats only.
/// 
#[derive(Enum, Copy, Clone, Eq, PartialEq)] //, Default
pub enum SerialFormat
{

    Abomonation,
    Bare,
    Bincode,
    Borsh,
    Postcard,
    Ciborium,
    MessagePack,
    Pickle,
    Flexbuffers,
    Bendy,
    DLHN

}

#[derive(InputObject, SimpleObject, Clone)] //, Default
#[graphql(input_name = "SerialDataInput")]
pub struct SerialData
{

    format: SerialFormat,
    data: Vec<u8>

}

//Output

#[derive(Union)]
pub enum AnyObject
{

    Bool(BoolValue),
    //Bool(bool),
    Char(CharValue),
    #[graphql(flatten)]
    Numeric(Numeric),
    String(StringValue),
    Unit(UnitValue), //(String), //
    //Unit(unit),
    //Unit(UnitEnum),
    Identifier(Identifier),
    //Serial(SerialData)

    //time - https://doc.rust-lang.org/std/time/index.html,

    //non-zero integers - https://doc.rust-lang.org/std/num/index.html

}

/*
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum UnitEnum
{

    Unit

}
*/

//Input

//InputObject can only be applied to an struct.rustc

//#[derive(Union)] //, InputObject)]

#[derive(OneofObject)]
pub enum AnyInputObject
{

    //Bool(Bool),
    //Char(Char),
    //#[graphql(flatten)]
    //Numeric(Numeric),
    //String(StringValue),
    //Unit(UnitValue),
    //Identifier(Identifier),
    //Serial(SerialData)

    //time - https://doc.rust-lang.org/std/time/index.html,

    //non-zero integers - https://doc.rust-lang.org/std/num/index.html

    Bool(bool),
    Char(char),
    //Numeric(Numeric),

    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    I128(I128Scalar),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    U128(U128Scalar),
    Usize(usize),

    //

    String(StringValue),
    //Unit(UnitValue),
    Unit(UnitInput),
    Identifier(Identifier),

}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)] 
pub enum UnitInput
{

    #[default]
    Unit
    
}

/*
#[derive(InputObject)]
pub struct AnyObjectToInput
{

    object: AnyInputObject

}
*/

