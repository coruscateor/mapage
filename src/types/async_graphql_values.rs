use std::{ops::*, str::FromStr, fmt::{Binary, Display, LowerExp, Octal, UpperExp, LowerHex, UpperHex, self}, num::ParseIntError, iter::{Product, Sum}};

use async_graphql::*;
use serde::{Serialize, Deserialize};
use tokio::io::copy;

//use crate::{identifier::*, consts::*};

//use super::unit_value::*;

use paste::paste;

#[macro_export]
macro_rules! impl_from_trait_value
{

    ($value_type:ty, $type_label:ident) =>
    {

        paste! {

            impl From<$value_type> for [<$type_label Value>]
            {

                fn from(value: $value_type) -> Self
                {

                    [<$type_label Value>]::new(value)
                    
                }

            }

        }

    }

}

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

/*
impl From<bool> for BoolValue
{

    fn from(value: bool) -> Self
    {

        BoolValue::new(value)
        
    }

}
*/

impl_from_trait_value!(bool, Bool);

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

/*
impl From<char> for CharValue
{

    fn from(value: char) -> Self
    {

        CharValue::new(value)
        
    }

}
*/

impl_from_trait_value!(char, Char);

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

/*
impl From<f32> for F32Value
{

    fn from(value: f32) -> Self
    {

        F32Value::new(value)
        
    }

}
*/

impl_from_trait_value!(f32, F32);

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

/*
impl From<f64> for F64Value
{

    fn from(value: f64) -> Self
    {

        F64Value::new(value)
        
    }

}
*/

impl_from_trait_value!(f64, F64);

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

/*
impl From<i8> for I8Value
{

    fn from(value: i8) -> Self
    {

        I8Value::new(value)
        
    }

}
*/

impl_from_trait_value!(i8, I8);

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

/*
impl From<i16> for I16Value
{

    fn from(value: i16) -> Self
    {

        I16Value::new(value)
        
    }

}
*/

impl_from_trait_value!(i16, I16);

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

/*
impl From<i32> for I32Value
{

    fn from(value: i32) -> Self
    {

        I32Value::new(value)
        
    }

}
*/

impl_from_trait_value!(i32, I32);

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

/*
impl From<i64> for I64Value
{

    fn from(value: i64) -> Self
    {

        I64Value::new(value)
        
    }

}
*/

impl_from_trait_value!(i64, I64);



//Send + Sync + Copy + 'static + Add<T> + AddAssign<T> + Binary + BitAnd<T> + BitAndAssign<T> + BitOr<T> + BitOrAssign<T> + BitXor<T> + BitXorAssign<T> + Clone + Default + Display + Div<T> + DivAssign<T> + FromStr + Hash + LowerExp + LowerHex + Mul<T> + MulAssign<T> + Neg + Not + Octal + Ord + PartialEq<T> + PartialOrd<T> + Product<T> + Rem<T> + RemAssign<T> + Shl<T> + ShlAssign<T> + Shr<T> + ShrAssign<T> + Sub<T> + SubAssign<T> + Sum<T> + ToString + UpperExp

//+ Not

//Add<T>, AddAssign<T>, BitAnd<T> + BitAndAssign<T> + BitOr<T> + BitOrAssign<T> + BitXor<T> + BitXorAssign<T> + Div<T> + DivAssign<T> + Mul<T> + MulAssign<T> PartialEq<T> + PartialOrd<T> + Product<T> + Rem<T> + RemAssign<T> + Shl<T> + ShlAssign<T> + Shr<T> + ShrAssign<T> + Sub<T> + SubAssign<T> + Sum<T> +

//Clone + Default 

//Binary, Display, FromStr, LowerExp, LowerHex, Neg, Not,  Octal, Ord, ToString, UpperExp

//https://async-graphql.github.io/async-graphql/en/custom_scalars.html

macro_rules! impl_int_traits
{

    ($scalar_type:ty, $value_type:ty) =>
    {

        impl Add<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn add(self, rhs: $scalar_type) -> Self::Output
            {
                
                <$scalar_type>::new(self.0.add(rhs.0))

            }

        }

        impl AddAssign<$scalar_type> for $scalar_type
        {

            fn add_assign(&mut self, rhs: $scalar_type)
            {
                
                self.0.add_assign(rhs.0);

            }

        }

        impl Binary for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {

                std::fmt::Binary::fmt(&self.0, f)
                
            }

        }

        impl BitAnd<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn bitand(self, rhs: $scalar_type) -> Self::Output
            {
                
                <$scalar_type>::new(self.0.bitand(rhs.0))

            }

        }

        impl BitAndAssign<$scalar_type> for $scalar_type
        {

            fn bitand_assign(&mut self, rhs: $scalar_type)
            {

                self.0.bitand_assign(rhs.0)
                
            }

        }

        impl BitOr<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn bitor(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.bitor(rhs.0))
            
            }

        }

        impl BitOrAssign<$scalar_type> for $scalar_type
        {

            fn bitor_assign(&mut self, rhs: $scalar_type)
            {

                self.0.bitor_assign(rhs.0)
            
            }

        }

        impl BitXor<$scalar_type> for $scalar_type
        {
            type Output = Self;

            fn bitxor(self, rhs: $scalar_type) -> Self::Output
            {
                
                <$scalar_type>::new(self.0.bitxor(rhs.0))

            }

        }

        impl BitXorAssign<$scalar_type> for $scalar_type
        {

            fn bitxor_assign(&mut self, rhs: $scalar_type)
            {

                self.0.bitxor_assign(rhs.0)

            }

        }

        impl Display for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {

                std::fmt::Display::fmt(&self.0, f)
                
            }

        }

        impl Div<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn div(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.div(rhs.0))      
                
            }

        }

        impl DivAssign<$scalar_type> for $scalar_type
        {

            fn div_assign(&mut self, rhs: $scalar_type)
            {
                
                self.0.div_assign(rhs.0)

            }

        }

        impl FromStr for $scalar_type
        {

            type Err = ParseIntError;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
            {
                
                Ok(<$scalar_type>::new(<$value_type>::from_str(s)?))

            }

        }

        impl LowerExp for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {

                LowerExp::fmt(&self.0, f)
                
            }

        }

        impl LowerHex for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {

                LowerHex::fmt(&self, f)

            }

        }

        impl Mul<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn mul(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.mul(rhs.0))

            }

        }

        impl MulAssign<$scalar_type> for $scalar_type
        {

            fn mul_assign(&mut self, rhs: $scalar_type)
            {
                
                self.0.mul_assign(rhs.0)

            }

        }

        //

        impl Not for $scalar_type
        {

            type Output = Self;

            fn not(self) -> Self::Output
            {

                <$scalar_type>::new(self.0.not())  

            }

        }

        impl Octal for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {

                Octal::fmt(&self.0, f)

            }

        }

        //

        /*
        impl Product<$scalar_type> for $scalar_type
        {

            fn product<I: Iterator<Item = $value_type>>(iter: I) -> Self
            {
                
                <$scalar_type>::new(<$value_type>::product(iter))

            }

        }
        */

        impl Rem<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn rem(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.rem(rhs.0))
                
            }

        }

        impl RemAssign<$scalar_type> for $scalar_type
        {

            fn rem_assign(&mut self, rhs: $scalar_type)
            {

                self.0.rem_assign(rhs.0)
                
            }

        }

        impl Shl<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn shl(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.shl(rhs.0))
                
            }

        }

        impl ShlAssign<$scalar_type> for $scalar_type
        {

            fn shl_assign(&mut self, rhs: $scalar_type)
            {
                
                self.0.shl_assign(rhs.0)

            }

        }

        impl Shr<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn shr(self, rhs: $scalar_type) -> Self::Output
            {
                
                <$scalar_type>::new(self.0.shr(rhs.0))

            }

        }

        impl ShrAssign<$scalar_type> for $scalar_type
        {

            fn shr_assign(&mut self, rhs: $scalar_type)
            {

                self.0.shr_assign(rhs.0)

            }

        }

        impl Sub<$scalar_type> for $scalar_type
        {

            type Output = Self;

            fn sub(self, rhs: $scalar_type) -> Self::Output
            {

                <$scalar_type>::new(self.0.sub(rhs.0))
                
            }

        }

        impl SubAssign<$scalar_type> for $scalar_type
        {

            fn sub_assign(&mut self, rhs: $scalar_type)
            {

                self.0.sub_assign(rhs.0)

            }

        }

        //

        impl UpperExp for $scalar_type
        {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                
                fmt::UpperExp::fmt(&self.0, f)

            }

        }
            
    }

}


//Custom Scalar

#[derive(Serialize, Deserialize, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)] //InputObject, 
pub struct I128Scalar(pub i128);

scalar!(I128Scalar);

impl I128Scalar
{
    
    pub fn new(value: i128) -> Self
    {

        Self(value)

        /*
        Self
        {

            let mut val = I128Scalar();

            I128Scalar(pvalue)

        }
        */

    }

}

impl_int_traits!(I128Scalar, i128);

/*
impl Add<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn add(self, rhs: I128Scalar) -> Self::Output
    {
        
        I128Scalar::new(self.0.add(rhs.0))

    }

}

impl AddAssign<I128Scalar> for I128Scalar
{

    fn add_assign(&mut self, rhs: I128Scalar)
    {
        
        self.0.add_assign(rhs.0);

    }

}

impl Binary for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        std::fmt::Binary::fmt(&self.0, f)
        
    }

}

impl BitAnd<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn bitand(self, rhs: I128Scalar) -> Self::Output
    {
        
        I128Scalar::new(self.0.bitand(rhs.0))

    }

}

impl BitAndAssign<I128Scalar> for I128Scalar
{

    fn bitand_assign(&mut self, rhs: I128Scalar)
    {

        self.0.bitand_assign(rhs.0)
        
    }

}

impl BitOr<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn bitor(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.bitor(rhs.0))
       
    }

}

impl BitOrAssign<I128Scalar> for I128Scalar
{

    fn bitor_assign(&mut self, rhs: I128Scalar)
    {

        self.0.bitor_assign(rhs.0)
       
    }

}

impl BitXor<I128Scalar> for I128Scalar
{
    type Output = Self;

    fn bitxor(self, rhs: I128Scalar) -> Self::Output
    {
        
        I128Scalar::new(self.0.bitxor(rhs.0))

    }

}

impl BitXorAssign<I128Scalar> for I128Scalar
{

    fn bitxor_assign(&mut self, rhs: I128Scalar)
    {

        self.0.bitxor_assign(rhs.0)

    }

}

impl Display for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        std::fmt::Display::fmt(&self.0, f)
        
    }

}

impl Div<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn div(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.div(rhs.0))      
        
    }

}

impl DivAssign<I128Scalar> for I128Scalar
{

    fn div_assign(&mut self, rhs: I128Scalar)
    {
        
        self.0.div_assign(rhs.0)

    }

}

impl FromStr for I128Scalar
{

    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
    {
        
        Ok(I128Scalar::new(i128::from_str(s)?))

    }

}

impl LowerExp for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        LowerExp::fmt(&self.0, f)
        
    }

}

impl LowerHex for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        LowerHex::fmt(&self, f)

    }

}

impl Mul<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn mul(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.mul(rhs.0))

    }

}

impl MulAssign<I128Scalar> for I128Scalar
{

    fn mul_assign(&mut self, rhs: I128Scalar)
    {
        
        self.0.mul_assign(rhs.0)

    }

}
*/

////

impl Neg for I128Scalar
{

    type Output = Self;

    fn neg(self) -> Self::Output
    {
        
        I128Scalar::new(self.0.neg())

    }

}

////

/*
impl Not for I128Scalar
{

    type Output = Self;

    fn not(self) -> Self::Output
    {

        I128Scalar::new(self.0.not())  

    }

}

impl Octal for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        Octal::fmt(&self.0, f)

    }

}
*/

/*
impl Ord for I128Scalar
{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {

        self.0.cmp(other)
        
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
        Self: ~const std::marker::Destruct,
    {
        match self.cmp(&other) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => other,
            std::cmp::Ordering::Greater => self,
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
        Self: ~const std::marker::Destruct,
    {
        match self.cmp(&other) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => self,
            std::cmp::Ordering::Greater => other,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: ~const std::marker::Destruct,
        Self: ~const PartialOrd,
    {
        assert!(min <= max);
        if self < std::cmp::min {
            std::cmp::min
        } else if self > std::cmp::max {
            std::cmp::max
        } else {
            self
        }
    }

}
*/

//PartialEq<T>

//PartialOrd<T>

//

/*
impl Product<I128Scalar> for I128Scalar
{

    fn product<I: Iterator<Item = i128>>(iter: I) -> Self
    {
        
        I128Scalar::new(i128::product(iter))

    }

}
*/

/*
impl Rem<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn rem(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.rem(rhs.0))
        
    }

}

impl RemAssign<I128Scalar> for I128Scalar
{

    fn rem_assign(&mut self, rhs: I128Scalar)
    {

        self.0.rem_assign(rhs.0)
        
    }

}

impl Shl<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn shl(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.shl(rhs.0))
        
    }

}

impl ShlAssign<I128Scalar> for I128Scalar
{

    fn shl_assign(&mut self, rhs: I128Scalar)
    {
        
        self.0.shl_assign(rhs.0)

    }

}

impl Shr<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn shr(self, rhs: I128Scalar) -> Self::Output
    {
        
        I128Scalar::new(self.0.shr(rhs.0))

    }

}

impl ShrAssign<I128Scalar> for I128Scalar
{

    fn shr_assign(&mut self, rhs: I128Scalar)
    {

        self.0.shr_assign(rhs.0)

    }

}

impl Sub<I128Scalar> for I128Scalar
{

    type Output = Self;

    fn sub(self, rhs: I128Scalar) -> Self::Output
    {

        I128Scalar::new(self.0.sub(rhs.0))
        
    }

}

impl SubAssign<I128Scalar> for I128Scalar
{

    fn sub_assign(&mut self, rhs: I128Scalar)
    {

        self.0.sub_assign(rhs.0)

    }

}
*/

/*
impl Sum<I128Scalar> for I128Scalar
{

    fn sum<I: Iterator<Item = I128Scalar>>(iter: I) -> Self
    {
        
        i128::sum(iter)

    }

}
*/

/*
impl ToString for I128Scalar
{

    fn to_string(&self) -> String
    {

        self.0.to_string()
        
    }

}
*/

/*
impl UpperExp for I128Scalar
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        fmt::UpperExp::fmt(&self.0, f)

    }

}
*/


//impl op traits

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

//#[derive(InputObject, SimpleObject, Default, Clone, Copy)]
#[derive(InputObject, SimpleObject, Default, Clone, Copy, Hash)]
#[graphql(input_name = "I28ValueInput")] //"I28ScalarValueInput")]
pub struct I128Value //I128ScalarValue
{

    //pub value: i128

    //pub value: String

    pub value: I128Scalar

}

impl I128Value //I128ScalarValue
{
    
    pub fn new(value: I128Scalar) -> Self
    {

        Self
        {

            value

        }

    }

    pub fn from_int(value: i128) -> Self
    {

        Self
        {

            value: I128Scalar(value)

        }

    }

}

impl_from_trait_value!(I128Scalar, I128); //I128Scalar); //I128Value

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

impl_from_trait_value!(isize, ISize);

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

impl_from_trait_value!(u8, U8);

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

impl_from_trait_value!(u16, U16);

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

impl_from_trait_value!(u32, U32);

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

impl_from_trait_value!(u64, U64);

//Custom Scalar

#[derive(Serialize, Deserialize, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct U128Scalar(pub u128);

scalar!(U128Scalar);

impl U128Scalar
{
    
    pub fn new(value: u128) -> Self
    {

        Self(value)

    }

}

impl_int_traits!(U128Scalar, u128);

//impl op traits

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


#[derive(InputObject, SimpleObject, Default, Clone, Copy, Hash)]
#[graphql(input_name = "U128ValueInput")] //"U128ScalarValueInput")]
pub struct U128Value //U128ScalarValue
{

    //pub value: i128

    //pub value: String

    pub value: U128Scalar

}

impl U128Value //U128ScalarValue
{
    
    pub fn new(value: U128Scalar) -> Self
    {

        Self
        {

            value

        }

    }

    pub fn from_int(value: u128) -> Self
    {

        Self
        {

            value: U128Scalar(value)

        }

    }

}

//impl_from_trait_value!(U128Scalar, U128Scalar);

impl_from_trait_value!(U128Scalar, U128);

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

impl_from_trait_value!(usize, USize);

/*
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
    //Identifier(Identifier)

}

#[derive(Union, Clone, Copy)]
pub enum NumericOrBool
{

    #[graphql(flatten)]
    Numeric(Numeric),
    Bool(BoolValue)

}
*/

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

impl_from_trait_value!(String, String);

//Output

#[derive(Union, Clone)] //, Default)]
pub enum Whatever //AnyObject
{

    //#[default]
    Bool(BoolValue),
    Char(CharValue),
    //#[graphql(flatten)]
    //Numeric(Numeric),
    F32(F32Value),
    F64(F64Value),
    I8(I8Value),
    I16(I16Value),
    I32(I32Value),
    I64(I64Value),

    I128(I128Value), //(I128ScalarValue),
    Isize(ISizeValue),
    U8(U8Value),
    U16(U16Value),
    U32(U32Value),
    U64(U64Value),

    U128(U128Value), //(U128ScalarValue),
    Usize(USizeValue),

    //Collections

    String(StringValue),

}

impl Default for Whatever
{

    fn default() -> Self
    {
        
        Whatever::Bool(BoolValue::default())

    }

}

impl From<InputOneofWhatever> for Whatever
{

    fn from(from_value: InputOneofWhatever) -> Self {

        match from_value
        {
            InputOneofWhatever::Bool(val) => Whatever::Bool(val.into()),
            InputOneofWhatever::Char(val) => Whatever::Char(val.into()),
            InputOneofWhatever::F32(val) => Whatever::F32(val.into()),
            InputOneofWhatever::F64(val) => Whatever::F64(val.into()),
            InputOneofWhatever::I8(val) => Whatever::I8(val.into()),
            InputOneofWhatever::I16(val) => Whatever::I16(val.into()),
            InputOneofWhatever::I32(val) => Whatever::I32(val.into()),
            InputOneofWhatever::I64(val) => Whatever::I64(val.into()),
            InputOneofWhatever::I128(val) => Whatever::I128(val.into()),
            InputOneofWhatever::Isize(val) => Whatever::Isize(val.into()),
            InputOneofWhatever::U8(val) => Whatever::U8(val.into()),
            InputOneofWhatever::U16(val) => Whatever::U16(val.into()),
            InputOneofWhatever::U32(val) => Whatever::U32(val.into()),
            InputOneofWhatever::U64(val) => Whatever::U64(val.into()),
            InputOneofWhatever::U128(val) => Whatever::U128(val.into()),
            InputOneofWhatever::Usize(val) => Whatever::Usize(val.into()),
            InputOneofWhatever::String(val) => Whatever::String(val.into()),
        }

    }

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

#[derive(OneofObject, Clone)] //, Hash
pub enum InputOneofWhatever //VariousObject
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

    //Collections

    String(String) //Value),

}

impl From<Whatever> for InputOneofWhatever
{

    fn from(from_value: Whatever) -> Self {
        
        match from_value
        {
            Whatever::Bool(val) => InputOneofWhatever::Bool(val.value),
            Whatever::Char(val) => InputOneofWhatever::Char(val.value),
            Whatever::F32(val) => InputOneofWhatever::F32(val.value),
            Whatever::F64(val) => InputOneofWhatever::F64(val.value),
            Whatever::I8(val) => InputOneofWhatever::I8(val.value),
            Whatever::I16(val) => InputOneofWhatever::I16(val.value),
            Whatever::I32(val) => InputOneofWhatever::I32(val.value),
            Whatever::I64(val) => InputOneofWhatever::I64(val.value),
            Whatever::I128(val) => InputOneofWhatever::I128(val.value),
            Whatever::Isize(val) => InputOneofWhatever::Isize(val.value),
            Whatever::U8(val) => InputOneofWhatever::U8(val.value),
            Whatever::U16(val) => InputOneofWhatever::U16(val.value),
            Whatever::U32(val) => InputOneofWhatever::U32(val.value),
            Whatever::U64(val) => InputOneofWhatever::U64(val.value),
            Whatever::U128(val) => InputOneofWhatever::U128(val.value),
            Whatever::Usize(val) => InputOneofWhatever::Usize(val.value),
            Whatever::String(val) => InputOneofWhatever::String(val.value),
        }

    }

}

/*
#[derive(Enum, Copy, Clone, Eq, PartialEq, Default)] 
pub enum UnitInput
{

    #[default]
    Unit
    
}
*/

/*

#[derive(Enum, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WhateverType
{

    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,

    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,

    U128,
    Usize,

    //Collections

    String

}
*/

//Selected - Output

cfg_if::cfg_if! {
    //if #[cfg(any(feature = "all_types", feature = "bool", feature = "char", feature = "f32", feature = "f64", feature = "i8", feature = "i16", feature = "i32", feature = "i64", feature = "i128", feature = "isize", feature = "u8", feature = "u16", feature = "u32", feature = "u64", feature = "u128", feature = "usize", feature = "String"))] {
        if #[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] {

        #[derive(Union, Clone)] //, FromStr
        pub enum SelectedType
        {

            #[cfg(any(feature = "all_types", feature = "bool"))]
            Bool(BoolValue),
            #[cfg(any(feature = "all_types", feature = "char"))]
            Char(CharValue),
            #[cfg(any(feature = "all_types", feature = "f32"))]
            F32(F32Value),
            #[cfg(any(feature = "all_types", feature = "f64"))]
            F64(F64Value),
            #[cfg(any(feature = "all_types", feature = "i8"))]
            I8(I8Value),
            #[cfg(any(feature = "all_types", feature = "i16"))]
            I16(I16Value),
            #[cfg(any(feature = "all_types", feature = "i32"))]
            I32(I32Value),
            #[cfg(any(feature = "all_types", feature = "i64"))]
            I64(I64Value),

            #[cfg(any(feature = "all_types", feature = "i128"))]
            I128(I128Value), //(I128ScalarValue),
            #[cfg(any(feature = "all_types", feature = "isize"))]
            ISize(ISizeValue),
            #[cfg(any(feature = "all_types", feature = "u8"))]
            U8(U8Value),
            #[cfg(any(feature = "all_types", feature = "u16"))]
            U16(U16Value),
            #[cfg(any(feature = "all_types", feature = "u32"))]
            U32(U32Value),
            #[cfg(any(feature = "all_types", feature = "u64"))]
            U64(U64Value),

            #[cfg(any(feature = "all_types", feature = "u128"))]
            U128(U128Value), //(U128ScalarValue),
            #[cfg(any(feature = "all_types", feature = "usize"))]
            USize(USizeValue),

            //Collections

            #[cfg(any(feature = "all_types", feature = "String"))]
            String(StringValue),

            //Whatever

            //#[cfg(any(feature = "all_types", feature = "Whatever"))]
            //Whatever(InputOneofWhatever)

            //Seems that Unions do not support OneofObjects

            //#[cfg(any(feature = "all_types", feature = "Whatever"))]
            //Whatever(Whatever)

        }

        impl Default for SelectedType
        {

            fn default() -> Self
            {
                
                SelectedType::Bool(BoolValue::default())

            }

        }

        impl From<InputOneofSelectedType> for SelectedType
        {

            fn from(from_value: InputOneofSelectedType) -> Self {

                match from_value
                {
                    #[cfg(any(feature = "all_types", feature = "bool"))]
                    InputOneofSelectedType::Bool(val) => SelectedType::Bool(val.into()),
                    #[cfg(any(feature = "all_types", feature = "char"))]
                    InputOneofSelectedType::Char(val) => SelectedType::Char(val.into()),
                    #[cfg(any(feature = "all_types", feature = "f32"))]
                    InputOneofSelectedType::F32(val) => SelectedType::F32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "f64"))]
                    InputOneofSelectedType::F64(val) => SelectedType::F64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i8"))]
                    InputOneofSelectedType::I8(val) => SelectedType::I8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i16"))]
                    InputOneofSelectedType::I16(val) => SelectedType::I16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i32"))]
                    InputOneofSelectedType::I32(val) => SelectedType::I32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i64"))]
                    InputOneofSelectedType::I64(val) => SelectedType::I64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i128"))]
                    InputOneofSelectedType::I128(val) => SelectedType::I128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "isize"))]
                    InputOneofSelectedType::Isize(val) => SelectedType::ISize(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u8"))]
                    InputOneofSelectedType::U8(val) => SelectedType::U8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u16"))]
                    InputOneofSelectedType::U16(val) => SelectedType::U16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u32"))]
                    InputOneofSelectedType::U32(val) => SelectedType::U32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u64"))]
                    InputOneofSelectedType::U64(val) => SelectedType::U64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u128"))]
                    InputOneofSelectedType::U128(val) => SelectedType::U128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "usize"))]
                    InputOneofSelectedType::Usize(val) => SelectedType::USize(val.into()),
                    #[cfg(any(feature = "all_types", feature = "string"))]
                    InputOneofSelectedType::String(val) => SelectedType::String(val.into()),
                }

            }

        }

        //time - https://doc.rust-lang.org/std/time/index.html,

        //non-zero integers - https://doc.rust-lang.org/std/num/index.html

        #[derive(OneofObject)]
        pub enum InputOneofSelectedType
        {
            #[cfg(any(feature = "all_types", feature = "bool"))]
            Bool(bool),
            #[cfg(any(feature = "all_types", feature = "char"))]
            Char(char),
            #[cfg(any(feature = "all_types", feature = "f32"))]
            F32(f32),
            #[cfg(any(feature = "all_types", feature = "f64"))]
            F64(f64),
            #[cfg(any(feature = "all_types", feature = "i8"))]
            I8(i8),
            #[cfg(any(feature = "all_types", feature = "i16"))]
            I16(i16),
            #[cfg(any(feature = "all_types", feature = "i32"))]
            I32(i32),
            #[cfg(any(feature = "all_types", feature = "i64"))]
            I64(i64),
            #[cfg(any(feature = "all_types", feature = "i128"))]
            I128(I128Scalar),
            #[cfg(any(feature = "all_types", feature = "isize"))]
            Isize(isize),
            #[cfg(any(feature = "all_types", feature = "u8"))]
            U8(u8),
            #[cfg(any(feature = "all_types", feature = "u16"))]
            U16(u16),
            #[cfg(any(feature = "all_types", feature = "u32"))]
            U32(u32),
            #[cfg(any(feature = "all_types", feature = "u64"))]
            U64(u64),
            #[cfg(any(feature = "all_types", feature = "u128"))]
            U128(U128Scalar),
            #[cfg(any(feature = "all_types", feature = "usize"))]
            Usize(usize),

            //Collections

            #[cfg(any(feature = "all_types", feature = "string"))]
            String(String)

        }

        impl From<SelectedType> for InputOneofSelectedType
        {

            fn from(from_value: SelectedType) -> Self {
                
                match from_value
                {
                    #[cfg(any(feature = "all_types", feature = "bool"))]
                    SelectedType::Bool(val) => InputOneofSelectedType::Bool(val.value),
                    #[cfg(any(feature = "all_types", feature = "char"))]
                    SelectedType::Char(val) => InputOneofSelectedType::Char(val.value),
                    #[cfg(any(feature = "all_types", feature = "f32"))]
                    SelectedType::F32(val) => InputOneofSelectedType::F32(val.value),
                    #[cfg(any(feature = "all_types", feature = "f64"))]
                    SelectedType::F64(val) => InputOneofSelectedType::F64(val.value),
                    #[cfg(any(feature = "all_types", feature = "i8"))]
                    SelectedType::I8(val) => InputOneofSelectedType::I8(val.value),
                    #[cfg(any(feature = "all_types", feature = "i16"))]
                    SelectedType::I16(val) => InputOneofSelectedType::I16(val.value),
                    #[cfg(any(feature = "all_types", feature = "i32"))]
                    SelectedType::I32(val) => InputOneofSelectedType::I32(val.value),
                    #[cfg(any(feature = "all_types", feature = "i64"))]
                    SelectedType::I64(val) => InputOneofSelectedType::I64(val.value),
                    #[cfg(any(feature = "all_types", feature = "i128"))]
                    SelectedType::I128(val) => InputOneofSelectedType::I128(val.value),
                    #[cfg(any(feature = "all_types", feature = "isize"))]
                    SelectedType::ISize(val) => InputOneofSelectedType::Isize(val.value),
                    #[cfg(any(feature = "all_types", feature = "u8"))]
                    SelectedType::U8(val) => InputOneofSelectedType::U8(val.value),
                    #[cfg(any(feature = "all_types", feature = "u16"))]
                    SelectedType::U16(val) => InputOneofSelectedType::U16(val.value),
                    #[cfg(any(feature = "all_types", feature = "u32"))]
                    SelectedType::U32(val) => InputOneofSelectedType::U32(val.value),
                    #[cfg(any(feature = "all_types", feature = "u64"))]
                    SelectedType::U64(val) => InputOneofSelectedType::U64(val.value),
                    #[cfg(any(feature = "all_types", feature = "u128"))]
                    SelectedType::U128(val) => InputOneofSelectedType::U128(val.value),
                    #[cfg(any(feature = "all_types", feature = "usize"))]
                    SelectedType::USize(val) => InputOneofSelectedType::Usize(val.value),
                    #[cfg(any(feature = "all_types", feature = "string"))]
                    SelectedType::String(val) => InputOneofSelectedType::String(val.value),
                }

            }

        }

        #[derive(Enum, Copy, Clone, Eq, PartialEq, Hash)] //, ToString
        pub enum AvalibleSelectedType
        {

            #[cfg(any(feature = "all_types", feature = "bool"))]
            Bool,
            #[cfg(any(feature = "all_types", feature = "char"))]
            Char,
            #[cfg(any(feature = "all_types", feature = "f32"))]
            F32,
            #[cfg(any(feature = "all_types", feature = "f64"))]
            F64,
            #[cfg(any(feature = "all_types", feature = "i8"))]
            I8,
            #[cfg(any(feature = "all_types", feature = "i16"))]
            I16,
            #[cfg(any(feature = "all_types", feature = "i32"))]
            I32,
            #[cfg(any(feature = "all_types", feature = "i64"))]
            I64,

            #[cfg(any(feature = "all_types", feature = "i128"))]
            I128,
            #[cfg(any(feature = "all_types", feature = "isize"))]
            ISize,
            #[cfg(any(feature = "all_types", feature = "u8"))]
            U8,
            #[cfg(any(feature = "all_types", feature = "u16"))]
            U16,
            #[cfg(any(feature = "all_types", feature = "u32"))]
            U32,
            #[cfg(any(feature = "all_types", feature = "u64"))]
            U64,

            #[cfg(any(feature = "all_types", feature = "u128"))]
            U128,
            #[cfg(any(feature = "all_types", feature = "usize"))]
            USize,

            //Collections

            #[cfg(any(feature = "all_types", feature = "String"))]
            String

        }

        impl TryFrom<String> for AvalibleSelectedType
        {

            type Error = async_graphql::Error;

            fn try_from(value: String) -> Result<Self, Self::Error>
            {

                match value.as_str()
                {

                    #[cfg(any(feature = "all_types", feature = "bool"))]
                    "Bool" => Ok(AvalibleSelectedType::Bool),
                    #[cfg(any(feature = "all_types", feature = "char"))]
                    "Char" => Ok(AvalibleSelectedType::Char),
                    #[cfg(any(feature = "all_types", feature = "f32"))]
                    "F32" => Ok(AvalibleSelectedType::F32),
                    #[cfg(any(feature = "all_types", feature = "f64"))]
                    "F64" => Ok(AvalibleSelectedType::F64),
                    #[cfg(any(feature = "all_types", feature = "i8"))]
                    "I8" => Ok(AvalibleSelectedType::I8),
                    #[cfg(any(feature = "all_types", feature = "i16"))]
                    "I16" => Ok(AvalibleSelectedType::I16),
                    #[cfg(any(feature = "all_types", feature = "i32"))]
                    "I32" => Ok(AvalibleSelectedType::I32),
                    #[cfg(any(feature = "all_types", feature = "i64"))]
                    "I64" => Ok(AvalibleSelectedType::I64),
        
                    #[cfg(any(feature = "all_types", feature = "i128"))]
                    "I128" => Ok(AvalibleSelectedType::I128),
                    #[cfg(any(feature = "all_types", feature = "isize"))]
                    "ISize" => Ok(AvalibleSelectedType::ISize),
                    #[cfg(any(feature = "all_types", feature = "u8"))]
                    "U8" => Ok(AvalibleSelectedType::U8),
                    #[cfg(any(feature = "all_types", feature = "u16"))]
                    "U16" => Ok(AvalibleSelectedType::U16),
                    #[cfg(any(feature = "all_types", feature = "u32"))]
                    "U32" => Ok(AvalibleSelectedType::U32),
                    #[cfg(any(feature = "all_types", feature = "u64"))]
                    "U64" => Ok(AvalibleSelectedType::U64),
        
                    #[cfg(any(feature = "all_types", feature = "u128"))]
                    "U128" => Ok(AvalibleSelectedType::U128),
                    #[cfg(any(feature = "all_types", feature = "usize"))]
                    "USize" => Ok(AvalibleSelectedType::USize),
        
                    //Collections
        
                    #[cfg(any(feature = "all_types", feature = "String"))]
                    "String" => Ok(AvalibleSelectedType::String),

                    //Error

                    _ => Err(Error::new("Error: Invalid type provided"))       

                }

            }

        }

        /*

        Originaly for:

        SelectedTypeIOQuery -> async fn read_selected_type_values

         */

        #[derive(SimpleObject)]
        pub struct SelectedTypeKvP
        {

            key: String,
            value: SelectedType

        }

        impl SelectedTypeKvP
        {

            pub fn new(key: String, value: SelectedType) -> Self
            {

                Self
                {

                    key,
                    value

                }

            }

        }

    }
}
