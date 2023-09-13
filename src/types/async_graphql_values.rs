use std::{ops::*, str::FromStr, fmt::{Binary, Display, LowerExp, Octal, UpperExp, LowerHex, UpperHex, self}, num::ParseIntError, iter::{Product, Sum}};

use async_graphql::*;
use serde::{Serialize, Deserialize};
//use tokio::io::copy;

use paste::paste;

//#[macro_export]
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

//

//#[macro_export]
macro_rules! impl_union_value
{

    ($type_label:ident, $value_type:ty) =>
    {

        paste! {

            #[derive(InputObject, SimpleObject, Default, Clone)]
            #[graphql(input_name = "[<$type_label ValueInput>]")]
            pub struct [<$type_label Value>]
            {

                pub value: $value_type

            }

            impl [<$type_label Value>]
            {
                
                pub fn new(value: $value_type) -> Self
                {

                    Self
                    {

                        value

                    }

                }

            }

            impl_from_trait_value!($value_type, $type_label);

        }

    }

}

//

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

impl_from_trait_value!(i64, I64);

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

#[derive(Serialize, Deserialize, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct I128Scalar(pub i128);

scalar!(I128Scalar);

impl I128Scalar
{
    
    pub fn new(value: i128) -> Self
    {

        Self(value)

    }

}

impl_int_traits!(I128Scalar, i128);

impl Neg for I128Scalar
{

    type Output = Self;

    fn neg(self) -> Self::Output
    {
        
        I128Scalar::new(self.0.neg())

    }

}


#[derive(InputObject, SimpleObject, Default, Clone, Copy, Hash)]
#[graphql(input_name = "I28ValueInput")]
pub struct I128Value
{

    pub value: I128Scalar

}

impl I128Value
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

impl_from_trait_value!(I128Scalar, I128);

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

#[derive(InputObject, SimpleObject, Default, Clone, Copy, Hash)]
#[graphql(input_name = "U128ValueInput")]
pub struct U128Value
{

    pub value: U128Scalar

}

impl U128Value
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

//

impl_union_value!(VecBool, Vec<bool>);

impl_union_value!(VecChar, Vec<char>);

impl_union_value!(VecF32, Vec<f32>);

impl_union_value!(VecF64, Vec<f64>);

impl_union_value!(VecI8, Vec<i8>);

impl_union_value!(VecI16, Vec<i16>);

impl_union_value!(VecI32, Vec<i32>);

impl_union_value!(VecI64, Vec<i64>);

impl_union_value!(VecI128, Vec<I128Scalar>);

impl_union_value!(VecISize, Vec<isize>);

impl_union_value!(VecU8, Vec<u8>);

impl_union_value!(VecU16, Vec<u16>);

impl_union_value!(VecU32, Vec<u32>);

impl_union_value!(VecU64, Vec<u64>);

impl_union_value!(VecU128, Vec<U128Scalar>);

impl_union_value!(VecUSize, Vec<usize>);

impl_union_value!(VecString, Vec<String>);

//Output

#[derive(Union, Clone)]
pub enum Whatever
{

    Bool(BoolValue),
    Char(CharValue),
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
    USize(USizeValue),

    //Collections

    String(StringValue),

    //Vecs

    VecBool(VecBoolValue),
    VecChar(VecCharValue),

    VecF32(VecF32Value),
    VecF64(VecF64Value),
    VecI8(VecI8Value),
    VecI16(VecI16Value),
    VecI32(VecI32Value),
    VecI64(VecI64Value),

    VecI128(VecI128Value),
    VecISize(VecISizeValue),
    VecU8(VecU8Value),
    VecU16(VecU16Value),
    VecU32(VecU32Value),
    VecU64(VecU64Value),

    VecU128(VecU128Value),
    VecUSize(VecUSizeValue),

    VecString(VecStringValue)

}

impl Default for Whatever
{

    fn default() -> Self
    {
        
        Whatever::Bool(BoolValue::default())

    }

}

impl From<InputOneOfWhatever> for Whatever
{

    fn from(from_value: InputOneOfWhatever) -> Self {

        match from_value
        {
            InputOneOfWhatever::Bool(val) => Whatever::Bool(val.into()),
            InputOneOfWhatever::Char(val) => Whatever::Char(val.into()),
            InputOneOfWhatever::F32(val) => Whatever::F32(val.into()),
            InputOneOfWhatever::F64(val) => Whatever::F64(val.into()),
            InputOneOfWhatever::I8(val) => Whatever::I8(val.into()),
            InputOneOfWhatever::I16(val) => Whatever::I16(val.into()),
            InputOneOfWhatever::I32(val) => Whatever::I32(val.into()),
            InputOneOfWhatever::I64(val) => Whatever::I64(val.into()),
            InputOneOfWhatever::I128(val) => Whatever::I128(val.into()),
            InputOneOfWhatever::Isize(val) => Whatever::Isize(val.into()),
            InputOneOfWhatever::U8(val) => Whatever::U8(val.into()),
            InputOneOfWhatever::U16(val) => Whatever::U16(val.into()),
            InputOneOfWhatever::U32(val) => Whatever::U32(val.into()),
            InputOneOfWhatever::U64(val) => Whatever::U64(val.into()),
            InputOneOfWhatever::U128(val) => Whatever::U128(val.into()),
            InputOneOfWhatever::USize(val) => Whatever::USize(val.into()),

            //Collections

            InputOneOfWhatever::String(val) => Whatever::String(val.into()),

            //Vecs

            InputOneOfWhatever::VecBool(val) => Whatever::VecBool(val.into()),
            InputOneOfWhatever::VecChar(val) => Whatever::VecChar(val.into()),
            InputOneOfWhatever::VecF32(val) => Whatever::VecF32(val.into()),
            InputOneOfWhatever::VecF64(val) => Whatever::VecF64(val.into()),
            InputOneOfWhatever::VecI8(val) => Whatever::VecI8(val.into()),
            InputOneOfWhatever::VecI16(val) => Whatever::VecI16(val.into()),
            InputOneOfWhatever::VecI32(val) => Whatever::VecI32(val.into()),
            InputOneOfWhatever::VecI64(val) => Whatever::VecI64(val.into()),
            InputOneOfWhatever::VecI128(val) => Whatever::VecI128(val.into()),
            InputOneOfWhatever::VecISize(val) => Whatever::VecISize(val.into()),
            InputOneOfWhatever::VecU8(val) => Whatever::VecU8(val.into()),
            InputOneOfWhatever::VecU16(val) => Whatever::VecU16(val.into()),
            InputOneOfWhatever::VecU32(val) => Whatever::VecU32(val.into()),
            InputOneOfWhatever::VecU64(val) => Whatever::VecU64(val.into()),
            InputOneOfWhatever::VecU128(val) => Whatever::VecU128(val.into()),
            InputOneOfWhatever::VecUSize(val) => Whatever::VecUSize(val.into()),
            InputOneOfWhatever::VecString(val) => Whatever::VecString(val.into()),
        }

    }

}

#[derive(OneofObject, Clone)]
pub enum InputOneOfWhatever
{

    //time - https://doc.rust-lang.org/std/time/index.html,

    //non-zero integers - https://doc.rust-lang.org/std/num/index.html

    Bool(bool),
    Char(char),

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
    USize(usize),

    //Collections

    String(String),

    //Vecs

    VecBool(Vec<bool>),
    VecChar(Vec<char>),

    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),

    VecI128(Vec<I128Scalar>),
    VecISize(Vec<isize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),

    VecU128(Vec<U128Scalar>),
    VecUSize(Vec<usize>),

    VecString(Vec<String>)

}

impl From<Whatever> for InputOneOfWhatever
{

    fn from(from_value: Whatever) -> Self {
        
        match from_value
        {
            Whatever::Bool(val) => InputOneOfWhatever::Bool(val.value),
            Whatever::Char(val) => InputOneOfWhatever::Char(val.value),
            Whatever::F32(val) => InputOneOfWhatever::F32(val.value),
            Whatever::F64(val) => InputOneOfWhatever::F64(val.value),
            Whatever::I8(val) => InputOneOfWhatever::I8(val.value),
            Whatever::I16(val) => InputOneOfWhatever::I16(val.value),
            Whatever::I32(val) => InputOneOfWhatever::I32(val.value),
            Whatever::I64(val) => InputOneOfWhatever::I64(val.value),
            Whatever::I128(val) => InputOneOfWhatever::I128(val.value),
            Whatever::Isize(val) => InputOneOfWhatever::Isize(val.value),
            Whatever::U8(val) => InputOneOfWhatever::U8(val.value),
            Whatever::U16(val) => InputOneOfWhatever::U16(val.value),
            Whatever::U32(val) => InputOneOfWhatever::U32(val.value),
            Whatever::U64(val) => InputOneOfWhatever::U64(val.value),
            Whatever::U128(val) => InputOneOfWhatever::U128(val.value),
            Whatever::USize(val) => InputOneOfWhatever::USize(val.value),

            //Collectons

            Whatever::String(val) => InputOneOfWhatever::String(val.value),

            Whatever::VecBool(val) => InputOneOfWhatever::VecBool(val.value),
            Whatever::VecChar(val) => InputOneOfWhatever::VecChar(val.value),
        
            Whatever::VecF32(val) => InputOneOfWhatever::VecF32(val.value),
            Whatever::VecF64(val) => InputOneOfWhatever::VecF64(val.value),
            Whatever::VecI8(val) => InputOneOfWhatever::VecI8(val.value),
            Whatever::VecI16(val) => InputOneOfWhatever::VecI16(val.value),
            Whatever::VecI32(val) => InputOneOfWhatever::VecI32(val.value),
            Whatever::VecI64(val) => InputOneOfWhatever::VecI64(val.value),
        
            Whatever::VecI128(val) => InputOneOfWhatever::VecI128(val.value),
            Whatever::VecISize(val) => InputOneOfWhatever::VecISize(val.value),
            Whatever::VecU8(val) => InputOneOfWhatever::VecU8(val.value),
            Whatever::VecU16(val) => InputOneOfWhatever::VecU16(val.value),
            Whatever::VecU32(val) => InputOneOfWhatever::VecU32(val.value),
            Whatever::VecU64(val) => InputOneOfWhatever::VecU64(val.value),
        
            Whatever::VecU128(val) => InputOneOfWhatever::VecU128(val.value),
            Whatever::VecUSize(val) => InputOneOfWhatever::VecUSize(val.value),
        
            Whatever::VecString(val) => InputOneOfWhatever::VecString(val.value),
        }

    }

}

//Selected - Output

cfg_if::cfg_if! {

        if #[cfg(any(feature = "all_types", feature = "SelectedType", feature = "SelectedTypeIO"))] {

        #[derive(Union, Clone)]
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
            I128(I128Value),
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
            U128(U128Value),
            #[cfg(any(feature = "all_types", feature = "usize"))]
            USize(USizeValue),

            //Collections

            #[cfg(any(feature = "all_types", feature = "String"))]
            String(StringValue),

            //Vecs

            #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
            VecBool(VecBoolValue),
            #[cfg(any(feature = "all_types", feature = "Vec_char"))]
            VecChar(VecCharValue),

            #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
            VecF32(VecF32Value),
            #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
            VecF64(VecF64Value),
            #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
            VecI8(VecI8Value),
            #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
            VecI16(VecI16Value),
            #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
            VecI32(VecI32Value),
            #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
            VecI64(VecI64Value),
            #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
            VecI128(VecI128Value),
            #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
            VecISize(VecISizeValue),
            #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
            VecU8(VecU8Value),
            #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
            VecU16(VecU16Value),
            #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
            VecU32(VecU32Value),
            #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
            VecU64(VecU64Value),
            #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
            VecU128(VecU128Value),
            #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
            VecUSize(VecUSizeValue),
            #[cfg(any(feature = "all_types", feature = "Vec_String"))]
            VecString(VecStringValue)

        }

        impl Default for SelectedType
        {

            fn default() -> Self
            {
                
                SelectedType::Bool(BoolValue::default())

            }

        }

        impl From<InputOneOfSelectedType> for SelectedType
        {

            fn from(from_value: InputOneOfSelectedType) -> Self {

                match from_value
                {
                    #[cfg(any(feature = "all_types", feature = "bool"))]
                    InputOneOfSelectedType::Bool(val) => SelectedType::Bool(val.into()),
                    #[cfg(any(feature = "all_types", feature = "char"))]
                    InputOneOfSelectedType::Char(val) => SelectedType::Char(val.into()),
                    #[cfg(any(feature = "all_types", feature = "f32"))]
                    InputOneOfSelectedType::F32(val) => SelectedType::F32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "f64"))]
                    InputOneOfSelectedType::F64(val) => SelectedType::F64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i8"))]
                    InputOneOfSelectedType::I8(val) => SelectedType::I8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i16"))]
                    InputOneOfSelectedType::I16(val) => SelectedType::I16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i32"))]
                    InputOneOfSelectedType::I32(val) => SelectedType::I32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i64"))]
                    InputOneOfSelectedType::I64(val) => SelectedType::I64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "i128"))]
                    InputOneOfSelectedType::I128(val) => SelectedType::I128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "isize"))]
                    InputOneOfSelectedType::Isize(val) => SelectedType::ISize(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u8"))]
                    InputOneOfSelectedType::U8(val) => SelectedType::U8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u16"))]
                    InputOneOfSelectedType::U16(val) => SelectedType::U16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u32"))]
                    InputOneOfSelectedType::U32(val) => SelectedType::U32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u64"))]
                    InputOneOfSelectedType::U64(val) => SelectedType::U64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "u128"))]
                    InputOneOfSelectedType::U128(val) => SelectedType::U128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "usize"))]
                    InputOneOfSelectedType::Usize(val) => SelectedType::USize(val.into()),

                    //Collections

                    #[cfg(any(feature = "all_types", feature = "string"))]
                    InputOneOfSelectedType::String(val) => SelectedType::String(val.into()),

                    //Vecs

                    #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
                    InputOneOfSelectedType::VecBool(val) => SelectedType::VecBool(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_char"))]
                    InputOneOfSelectedType::VecChar(val) => SelectedType::VecChar(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
                    InputOneOfSelectedType::VecF32(val) => SelectedType::VecF32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
                    InputOneOfSelectedType::VecF64(val) => SelectedType::VecF64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
                    InputOneOfSelectedType::VecI8(val) => SelectedType::VecI8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
                    InputOneOfSelectedType::VecI16(val) => SelectedType::VecI16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
                    InputOneOfSelectedType::VecI32(val) => SelectedType::VecI32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
                    InputOneOfSelectedType::VecI64(val) => SelectedType::VecI64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
                    InputOneOfSelectedType::VecI128(val) => SelectedType::VecI128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
                    InputOneOfSelectedType::VecISize(val) => SelectedType::VecISize(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
                    InputOneOfSelectedType::VecU8(val) => SelectedType::VecU8(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
                    InputOneOfSelectedType::VecU16(val) => SelectedType::VecU16(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
                    InputOneOfSelectedType::VecU32(val) => SelectedType::VecU32(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
                    InputOneOfSelectedType::VecU64(val) => SelectedType::VecU64(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
                    InputOneOfSelectedType::VecU128(val) => SelectedType::VecU128(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
                    InputOneOfSelectedType::VecUSize(val) => SelectedType::VecUSize(val.into()),
                    #[cfg(any(feature = "all_types", feature = "Vec_String"))]
                    InputOneOfSelectedType::VecString(val) => SelectedType::VecString(val.into())

                }

            }

        }

        //time - https://doc.rust-lang.org/std/time/index.html,

        //non-zero integers - https://doc.rust-lang.org/std/num/index.html

        #[derive(OneofObject)]
        pub enum InputOneOfSelectedType
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
            String(String),

            //Vecs

            #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
            VecBool(Vec<bool>),
            #[cfg(any(feature = "all_types", feature = "Vec_char"))]
            VecChar(Vec<char>),

            #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
            VecF32(Vec<f32>),
            #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
            VecF64(Vec<f64>),
            #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
            VecI8(Vec<i8>),
            #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
            VecI16(Vec<i16>),
            #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
            VecI32(Vec<i32>),
            #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
            VecI64(Vec<i64>),
            #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
            VecI128(Vec<I128Scalar>),
            #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
            VecISize(Vec<isize>),
            #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
            VecU8(Vec<u8>),
            #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
            VecU16(Vec<u16>),
            #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
            VecU32(Vec<u32>),
            #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
            VecU64(Vec<u64>),
            #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
            VecU128(Vec<U128Scalar>),
            #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
            VecUSize(Vec<usize>),
            #[cfg(any(feature = "all_types", feature = "Vec_String"))]
            VecString(Vec<String>)
            

        }

        impl From<SelectedType> for InputOneOfSelectedType
        {

            fn from(from_value: SelectedType) -> Self {
                
                match from_value
                {
                    #[cfg(any(feature = "all_types", feature = "bool"))]
                    SelectedType::Bool(val) => InputOneOfSelectedType::Bool(val.value),
                    #[cfg(any(feature = "all_types", feature = "char"))]
                    SelectedType::Char(val) => InputOneOfSelectedType::Char(val.value),
                    #[cfg(any(feature = "all_types", feature = "f32"))]
                    SelectedType::F32(val) => InputOneOfSelectedType::F32(val.value),
                    #[cfg(any(feature = "all_types", feature = "f64"))]
                    SelectedType::F64(val) => InputOneOfSelectedType::F64(val.value),
                    #[cfg(any(feature = "all_types", feature = "i8"))]
                    SelectedType::I8(val) => InputOneOfSelectedType::I8(val.value),
                    #[cfg(any(feature = "all_types", feature = "i16"))]
                    SelectedType::I16(val) => InputOneOfSelectedType::I16(val.value),
                    #[cfg(any(feature = "all_types", feature = "i32"))]
                    SelectedType::I32(val) => InputOneOfSelectedType::I32(val.value),
                    #[cfg(any(feature = "all_types", feature = "i64"))]
                    SelectedType::I64(val) => InputOneOfSelectedType::I64(val.value),
                    #[cfg(any(feature = "all_types", feature = "i128"))]
                    SelectedType::I128(val) => InputOneOfSelectedType::I128(val.value),
                    #[cfg(any(feature = "all_types", feature = "isize"))]
                    SelectedType::ISize(val) => InputOneOfSelectedType::Isize(val.value),
                    #[cfg(any(feature = "all_types", feature = "u8"))]
                    SelectedType::U8(val) => InputOneOfSelectedType::U8(val.value),
                    #[cfg(any(feature = "all_types", feature = "u16"))]
                    SelectedType::U16(val) => InputOneOfSelectedType::U16(val.value),
                    #[cfg(any(feature = "all_types", feature = "u32"))]
                    SelectedType::U32(val) => InputOneOfSelectedType::U32(val.value),
                    #[cfg(any(feature = "all_types", feature = "u64"))]
                    SelectedType::U64(val) => InputOneOfSelectedType::U64(val.value),
                    #[cfg(any(feature = "all_types", feature = "u128"))]
                    SelectedType::U128(val) => InputOneOfSelectedType::U128(val.value),
                    #[cfg(any(feature = "all_types", feature = "usize"))]
                    SelectedType::USize(val) => InputOneOfSelectedType::Usize(val.value),

                    //Collections

                    #[cfg(any(feature = "all_types", feature = "string"))]
                    SelectedType::String(val) => InputOneOfSelectedType::String(val.value),

                    //Vecs

                    #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
                    SelectedType::VecBool(val) => InputOneOfSelectedType::VecBool(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_char"))]
                    SelectedType::VecChar(val) => InputOneOfSelectedType::VecChar(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
                    SelectedType::VecF32(val) => InputOneOfSelectedType::VecF32(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
                    SelectedType::VecF64(val) => InputOneOfSelectedType::VecF64(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
                    SelectedType::VecI8(val) => InputOneOfSelectedType::VecI8(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
                    SelectedType::VecI16(val) => InputOneOfSelectedType::VecI16(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
                    SelectedType::VecI32(val) => InputOneOfSelectedType::VecI32(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
                    SelectedType::VecI64(val) => InputOneOfSelectedType::VecI64(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
                    SelectedType::VecI128(val) => InputOneOfSelectedType::VecI128(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
                    SelectedType::VecISize(val) => InputOneOfSelectedType::VecISize(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
                    SelectedType::VecU8(val) => InputOneOfSelectedType::VecU8(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
                    SelectedType::VecU16(val) => InputOneOfSelectedType::VecU16(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
                    SelectedType::VecU32(val) => InputOneOfSelectedType::VecU32(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
                    SelectedType::VecU64(val) => InputOneOfSelectedType::VecU64(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
                    SelectedType::VecU128(val) => InputOneOfSelectedType::VecU128(val.value),
                    #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
                    SelectedType::VecUSize(val) => InputOneOfSelectedType::VecUSize(val.value),

                    //Collections

                    #[cfg(any(feature = "all_types", feature = "string"))]
                    SelectedType::VecString(val) => InputOneOfSelectedType::VecString(val.value),


                }

            }

        }

        #[derive(Enum, Copy, Clone, Eq, PartialEq, Hash)]
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
            String,

            //Vecs

            #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
            VecBool,
            #[cfg(any(feature = "all_types", feature = "Vec_char"))]
            VecChar,
            #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
            VecF32,
            #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
            VecF64,
            #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
            VecI8,
            #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
            VecI16,
            #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
            VecI32,
            #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
            VecI64,
            #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
            VecI128,
            #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
            VecISize,
            #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
            VecU8,
            #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
            VecU16,
            #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
            VecU32,
            #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
            VecU64,
            #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
            VecU128,
            #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
            VecUSize,
            #[cfg(any(feature = "all_types", feature = "Vec_string"))]
            VecString

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

                    //Vecs

                    #[cfg(any(feature = "all_types", feature = "Vec_bool"))]
                    "VecBool" => Ok(AvalibleSelectedType::VecBool),
                    #[cfg(any(feature = "all_types", feature = "Vec_char"))]
                    "VecChar" => Ok(AvalibleSelectedType::VecChar),
                    #[cfg(any(feature = "all_types", feature = "Vec_f32"))]
                    "VecF32" => Ok(AvalibleSelectedType::VecF32),
                    #[cfg(any(feature = "all_types", feature = "Vec_f64"))]
                    "VecF64" => Ok(AvalibleSelectedType::VecF64),
                    #[cfg(any(feature = "all_types", feature = "Vec_i8"))]
                    "VecI8" => Ok(AvalibleSelectedType::VecI8),
                    #[cfg(any(feature = "all_types", feature = "Vec_i16"))]
                    "VecI16" => Ok(AvalibleSelectedType::VecI16),
                    #[cfg(any(feature = "all_types", feature = "Vec_i32"))]
                    "VecI32" => Ok(AvalibleSelectedType::VecI32),
                    #[cfg(any(feature = "all_types", feature = "Vec_i64"))]
                    "VecI64" => Ok(AvalibleSelectedType::VecI64),
                    #[cfg(any(feature = "all_types", feature = "Vec_i128"))]
                    "VecI128" => Ok(AvalibleSelectedType::VecI128),
                    #[cfg(any(feature = "all_types", feature = "Vec_isize"))]
                    "VecISize" => Ok(AvalibleSelectedType::VecISize),
                    #[cfg(any(feature = "all_types", feature = "Vec_u8"))]
                    "VecU8" => Ok(AvalibleSelectedType::VecU8),
                    #[cfg(any(feature = "all_types", feature = "Vec_u16"))]
                    "VecU16" => Ok(AvalibleSelectedType::VecU16),
                    #[cfg(any(feature = "all_types", feature = "Vec_u32"))]
                    "VecU32" => Ok(AvalibleSelectedType::VecU32),
                    #[cfg(any(feature = "all_types", feature = "Vec_u64"))]
                    "VecU64" => Ok(AvalibleSelectedType::VecU64),
                    #[cfg(any(feature = "all_types", feature = "Vec_u128"))]
                    "VecU128" => Ok(AvalibleSelectedType::VecU128),
                    #[cfg(any(feature = "all_types", feature = "Vec_usize"))]
                    "VecUSize" => Ok(AvalibleSelectedType::VecUSize),
                    #[cfg(any(feature = "all_types", feature = "Vec_string"))]
                    "VecString" => Ok(AvalibleSelectedType::VecString),

                    //Error

                    _ => Err(Error::new("Error: Invalid type provided"))       

                }

            }

        }

        /*
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
        */

    }
}
