use crate::types::{numeric_values::NumericValue, unit_value::UnitValue};

//use super::macros::*;

use crate::impl_unwarp_mutex_field;

use corlib::has_one::*;

use std::{ops::*, default};



type Mtx<T> = std::sync::Mutex<T>;

#[derive(Default)]
pub struct MutexNumericValue<T, HO>
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
    HO: HasOne<T> + Default
{

    value: Mtx<NumericValue<T, HO>>

}

impl<T, HO> MutexNumericValue<T, HO>
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
    HO: HasOne<T> + Default
{

    //pub fn new(value: NumericValue<T, HO>) -> Self
    pub fn new(value: T) -> Self
    {

        Self
        {

                value: Mtx::new(NumericValue::<T, HO>::new(value)) //::default())
            
        }

    }
    
    /*
    pub fn init(value: T) -> Self
    {

        Self
        {

                value: mtx::new(NumericValue::<T, HO>::new(value))
            
        }

    }
    */

    impl_unwarp_mutex_field!(value, get_value, T);

    impl_unwarp_mutex_field!(value, set_value, value: T);

    impl_unwarp_mutex_field!(value, inc, T);

    impl_unwarp_mutex_field!(value, dec, T);

    impl_unwarp_mutex_field!(value, ne, T);

    impl_unwarp_mutex_field!(value, not, T);

    impl_unwarp_mutex_field!(value, add, T, right_side: T);

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

    //impl_unwarp_mutex_field!(value, bit_and, T, right_side: T);
    
    //impl_unwarp_mutex_field!(value, bit_or, T, right_side: T);

    //impl_unwarp_mutex_field!(value, bit_xor, T, right_side: T);

    impl_unwarp_mutex_field!(value, div, T, right_side: T);

    impl_unwarp_mutex_field!(value, mul, T, right_side: T);

    //impl_unwarp_mutex_field!(value, rem, T, right_side: T);

    //impl_unwarp_mutex_field!(value, shl, T, right_side: T);

    impl_unwarp_mutex_field!(value, shr, T, right_side: T);

    impl_unwarp_mutex_field!(value, sub, T, right_side: T);

}

impl<T, HO> From<T> for MutexNumericValue<T, HO>
    where T: Add + AddAssign + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Div + DivAssign + Mul + MulAssign + Neg<Output = T> + Not<Output = T> + Rem + RemAssign + Shl + ShlAssign + Shr + ShrAssign + Sub + SubAssign + Default + Copy,
    HO: HasOne<T> + Default
{

    fn from(value: T) -> Self
    {
        
       MutexNumericValue::<T, HO>::new(value) 
    
    }
    
}

