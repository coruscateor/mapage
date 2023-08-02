use std::ops::*;

use super::UnitValue;

use paste::paste;

use corlib::{impl_get};

use crate::impl_set;

use corlib::has_one::*;

use std::marker::PhantomData;

//https://doc.rust-lang.org/std/ops/index.html

//type numeric_contarints = dyn Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitOrAssign + Div + DivAssign + Mul + MulAssign + Neg + Not + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default;

/*
macro_rules! constraints
{

    () =>
    {

        T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitOrAssign + Div + DivAssign + Mul + MulAssign + Neg + Not + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default

    }

}
*/

#[derive(Default)]
pub struct NumericValue<T, HO>
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
          HO: HasOne<T> + Default
    //where constraints!
    //where T: numeric_contarints
{

    value : T,
    pt: PhantomData<HO>

}

impl<T, HO> NumericValue<T, HO>
    //where constraints!
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
          HO: HasOne<T> + Default
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            value,
            pt: PhantomData::default()

        }

    }

    impl_get!(value, T);

    impl_set!(value, T);

    //uops

    pub fn inc(&mut self) -> T
    {

        self.value += HO::one();

        self.value

    }

    pub fn dec(&mut self) -> T
    {

        self.value -= HO::one();

        self.value

    }

    pub fn ne(&mut self) -> T
    {

        self.value = -self.value;

        self.value

    }

    pub fn not(&mut self) -> T
    {

        self.value = !self.value;

        self.value

    }

    //bops

    pub fn add(&mut self, right_side: T) -> T
    {

        self.value += right_side;

        self.value

    }

        /*
        no implementation for `f32 & f32`
        the trait `std::ops::BitAnd` is not implemented for `f32`
        the following other types implement trait `std::ops::BitAnd<Rhs>`:
    <&'a i128 as std::ops::BitAnd<i128>>
    <&'a i16 as std::ops::BitAnd<i16>>
    <&'a i32 as std::ops::BitAnd<i32>>
    <&'a i64 as std::ops::BitAnd<i64>>
    <&'a i8 as std::ops::BitAnd<i8>>
    <&'a isize as std::ops::BitAnd<isize>>
    <&'a u128 as std::ops::BitAnd<u128>>
    <&'a u16 as std::ops::BitAnd<u16>>
    */

    /*
    pub fn bit_and(&mut self, right_side: T) -> T
    {

        self.value &= right_side;

        self.value

    }
    
    pub fn bit_or(&mut self, right_side: T) -> T
    {

        self.value |= right_side;

        self.value

    }
    
    pub fn bit_xor(&mut self, right_side: T) -> T
    {

        self.value ^= right_side;

        self.value

    }
    */

    pub fn div(&mut self, right_side: T) -> T
    {

        self.value /= right_side;

        self.value

    }

    pub fn mul(&mut self, right_side: T) -> T
    {

        self.value *= right_side;

        self.value

    }

    /*
    pub fn rem(&mut self, right_side: T) -> T
    {

        self.value %= right_side;

        self.value

    }

    pub fn shl(&mut self, right_side: T) -> T
    {

        self.value <<= right_side;

        self.value

    }
    */

    pub fn shr(&mut self, right_side: T) -> T
    {

        self.value >>= right_side;

        self.value

    }

    pub fn sub(&mut self, right_side: T) -> T
    {

        self.value -= right_side;

        self.value

    }

}

impl<T, HO> From<T> for NumericValue<T, HO>
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
          HO: HasOne<T> + Default
{

    fn from(value: T) -> Self
    {

        NumericValue::<T, HO>::new(value)

    }

}
