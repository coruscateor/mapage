use core::ops::*;

use corlib::has_one::*;

use super::UnitValue;

use paste::paste;

#[macro_export]
macro_rules! inc_fn
{

    ($label:ident, $field:ident, $value_type:ty) =>
    {

        paste! {

            pub fn [<$label _inc>](&mut value: $value_type) -> async_graphql::Result<UnitValue>
            {

                let res = self.$field.insert_async(key, value);

                if let Err(_) = res.await
                {

                    return Err(invalid_operation());

                }

                Ok(UnitValue::new())

            }

        }

    } 

}

pub fn inc<T, HO>(&mut value: T) -> T
{

    value += HO::one();

    value

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



