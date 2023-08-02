
//https://doc.rust-lang.org/std/ops/index.html

use core::ops::*;

use corlib::has_one::*;

//use super::UnitValue;

//use paste::paste;

use crate::errors::*;

/*
pub fn add<T: Add>(l: &mut T, r: T) -> T
{

    l += r;

    l

}
*/

//pub type MutClosure<T> = impl FnMut(&mut T) -> async_graphql::Result<T>;

//Add

//AddAssign

pub fn add<T: AddAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{

    *l += r;

    Ok(*l)

}

//, FN: FnMut(&mut T) -> async_graphql::Result<T>

pub fn add_fn<T: AddAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    move |l: &mut T| { add(l, r) }
    
}

pub fn add_self<T: AddAssign + Copy>(l: &mut T) -> async_graphql::Result<T>
{

    *l += *l;

    Ok(*l)

}

pub fn add_self_fn<T: AddAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { add_self(l) }
    
}

//BitAnd

//BitAndAssign

pub fn bit_and<T: BitAndAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{
    
    *l &= r;

    Ok(*l)

}

pub fn bit_and_fn<T: BitAndAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T,
{

    move |l: &mut T| { bit_and(l, r) }

}

pub fn bit_and_self<T: BitAndAssign + Copy>(l: &mut T) -> async_graphql::Result<T>
{
    
    *l &= *l;

    Ok(*l)

}

pub fn bit_and_self_fn<T: BitAndAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { bit_and_self(l) }
    
}

//BitOr

//BitOrAssign

pub fn bit_or<T: BitOrAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{
    
    *l |= r;

    Ok(*l)

}

pub fn bit_or_fn<T: BitOrAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { bit_or(l, r) }

}

pub fn bit_or_self<T: BitOrAssign + Copy>(l: &mut T) -> async_graphql::Result<T>
{
    
    *l |= *l;

    Ok(*l)

}

pub fn bit_or_self_fn<T: BitOrAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { bit_or_self(l) }
    
}


//BitXor

//BitXorAssign

pub fn bit_xor<T: BitXorAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{
    
    *l ^= r;

    Ok(*l)

}

pub fn bit_xor_fn<T: BitXorAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { bit_xor(l, r) }

}

pub fn bit_xor_self<T: BitXorAssign + Copy>(l: &mut T) -> async_graphql::Result<T>
{
    
    *l ^= *l;

    Ok(*l)

}

pub fn bit_xor_self_fn<T: BitXorAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { bit_xor_self(l) }
    
}


//Deref

//DerefMut



//Div

//DivAssign

//Obviosly default() return be zero

//NaN?

pub fn div<T: DivAssign + PartialEq + Default + Copy>(l: &mut T, r: T) -> async_graphql::Result<T> //, Z: Default HasOne<T>
{

    //check if l or r are zero

    if *l == T::default() //Z::one()
    {

        return stored_value_is_zero();

    }

    if r == T::default() //Z::one()
    {

        return provided_value_is_zero();

    }
    
    *l /= r;

    Ok(*l)

}

pub fn div_fn<T: DivAssign + PartialEq + Default + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { div(l, r) }

}

pub fn div_self<T: DivAssign + PartialEq + Default + Copy>(l: &mut T) -> async_graphql::Result<T>
{

    //check if l is zero

    if *l == T::default()
    {

        return stored_value_is_zero();

    }
    
    *l /= *l;

    Ok(*l)

}

pub fn div_self_fn<T: DivAssign + PartialEq + Default + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { div_self(l) }
    
}

//Fn

//FnMut

//FnOnce

//Index

/*
pub fn index<T: Index<I, Output: R>, I, R>(l: &T, indexer: I) -> R
{
    
    l[indexer];

}
*/

//IndexMut

/*
pub fn index_mut<T: IndexMut<I>, I, R>(l: &mut T, indexer: I) -> R
{
    
    l[indexer];

}
*/

//Mul

//MulAssign

pub fn mul<T: MulAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{
    
    *l *= r;

    Ok(*l)

}

pub fn mul_fn<T: MulAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { mul(l, r) }

}

pub fn mul_self<T: MulAssign + Copy>(l: &mut T) -> async_graphql::Result<T>
{
    
    *l *= *l;

    Ok(*l)

}

pub fn mul_self_fn<T: MulAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T>
{

    |l: &mut T| { mul_self(l) }
    
}


//Neg

pub fn neg<T: Neg<Output = T> + Copy>(l: &mut T) -> async_graphql::Result<T>
{

    //-*l

    *l = -*l;

    Ok(*l)

}

pub fn neg_fn<T: Neg<Output = T> + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { neg(l) }

}

//Not

pub fn not<T: Not<Output = T> + Copy>(value: &mut T) -> async_graphql::Result<T>
{

    *value = !*value;

    Ok(*value)

}

pub fn not_fn<T: Not<Output = T> + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { not(l) }

}

//RangeBounds

//https://doc.rust-lang.org/std/ops/trait.RangeBounds.html

/*
pub fn range_bounds<T: RangeBounds<I>, I>(value: &mut T, first: I, last: I) -> T
{

    (*value)(first..last)

}
*/

//Rem

//RemAssign

pub fn rem<T: RemAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{

    *l %= r;

    Ok(*l)

}

pub fn rem_fn<T: RemAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { rem(l, r) }

}

pub fn rem_self<T: RemAssign + Copy>(value: &mut T) -> async_graphql::Result<T>
{

    *value %= *value;

    Ok(*value)

}

pub fn rem_self_fn<T: RemAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { rem_self(l) }

}

//Shl

//ShlAssign

pub fn shl<T: ShlAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{

    *l <<= r;

    Ok(*l)

}

pub fn shl_fn<T: ShlAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { shl(l, r) }

}

pub fn shl_self<T: ShlAssign + Copy>(value: &mut T) -> async_graphql::Result<T>
{

    *value <<= *value;

    Ok(*value)

}

pub fn shl_self_fn<T: ShlAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { shl_self(l) }

}


//Shr

//ShrAssign

pub fn shr<T: ShrAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{

    *l >>= r;

    Ok(*l)

}

pub fn shr_fn<T: ShrAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { shr(l, r) }

}

pub fn shr_self<T: ShrAssign + Copy>(value: &mut T) -> async_graphql::Result<T>
{

    *value >>= *value;

    Ok(*value)

}

pub fn shr_self_fn<T: ShrAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { shr_self(l) }

}

//Sub

//SubAssign

pub fn sub<T: SubAssign + Copy>(l: &mut T, r: T) -> async_graphql::Result<T>
{

    *l -= r;

    Ok(*l)

}

pub fn sub_fn<T: SubAssign + Copy>(r: T) -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, 
{

    move |l: &mut T| { sub(l, r) }

}

pub fn sub_self<T: SubAssign + Copy>(value: &mut T) -> async_graphql::Result<T>
{

    *value -= *value;

    Ok(*value)

}

pub fn sub_self_fn<T: SubAssign + Copy>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T
{

    |l: &mut T| { sub_self(l) }

}

//

pub fn inc<T: AddAssign + Copy, HO: HasOne<T>>(value: &mut T) -> async_graphql::Result<T>
{

    *value += HO::one();

    Ok(*value)

}

pub fn inc_fn<T: AddAssign + Copy, HO: HasOne<T>>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //l: &mut T, r: T
{

    |value: &mut T| { inc::<T, HO>(value) }

}

pub fn dec<T: SubAssign + Copy, HO: HasOne<T>>(value: &mut T) -> async_graphql::Result<T>
{

    *value -= HO::one();

    Ok(*value)

}

pub fn dec_fn<T: SubAssign + Copy, HO: HasOne<T>>() -> impl FnMut(&mut T) -> async_graphql::Result<T> //value: &mut T
{

    |value: &mut T| { dec::<T, HO>(value) }

}

//Ops Macros

//use paste::paste;

#[macro_export]
macro_rules! impl_update_fn_op_method
{

    /*
    ($op:ident, $key_type:ty, $retun_value_type:ty $(,$parameter_name:ident: $parameter_type:ty)*) =>
    {

        paste! {

            pub async fn [<$op _update_op>](&self, key: &$key_type, $(,$parameter_name: $parameter_type)*) -> async_graphql::Result<$retun_value_type> //<T>//, value: &mut T //<T>
            {

                let op_inst = [<$op _fn>]($($parameter_name,)*);

                self.namespace.update_fn(key, op_inst).await

            }

        }

    };*/
    ($op:ident, $key_type:ty, $retun_value_type:ty $(,$parameter_name:ident: $parameter_type:ty)*) => //, $main_parameter:ident: $main_parameter_type:ty 
    {

        paste! {

            pub async fn [<$op _op>](&self, key: &$key_type $(,$parameter_name: $parameter_type)*) -> async_graphql::Result<$retun_value_type> //_update //<T>//, value: &mut T //<T> //, $main_parameter: &mut $main_parameter_type 
            {

                let op_inst = [<$op _fn>]($($parameter_name,)*); //$main_parameter 

                self.namespace.update_fn(key, op_inst).await

            }

        }

    };
    ($op:ident, $key_type:ty, $retun_value_type:ty, $has_one_type:ty $(,$parameter_name:ident: $parameter_type:ty)*) =>
    {

        paste! {

            pub async fn [<$op _op>](&self, key: &$key_type $(,$parameter_name: $parameter_type)*) -> async_graphql::Result<$retun_value_type> //_update
            {

                let op_inst = [<$op _fn>]::<$retun_value_type, $has_one_type>($($parameter_name,)*);

                self.namespace.update_fn(key, op_inst).await

            }

        }

    } 

}


//

//pub fn inc_f32()

//incrementation

/*
#[macro_export]
macro_rules! inc_fn
{

    ($value_type:ty, $has_one:ty) => //$label:ident, $
    {

        paste! {

            pub fn [<inc_ $value_type>](value: &mut $value_type) -> async_graphql::Result<$value_type>
            {

                *value += $has_one::one();

                Ok(*value)

            }

        }

    } 

}

inc_fn!(f32, F32HasOne);

inc_fn!(f64, F64HasOne);

//

inc_fn!(i8, I8HasOne);

inc_fn!(i16, I16HasOne);

inc_fn!(i32, I32HasOne);

inc_fn!(i64, I64HasOne);

//i128

inc_fn!(isize, ISizeHasOne);

//

inc_fn!(u8, U8HasOne);

inc_fn!(u16, U16HasOne);

inc_fn!(u32, U32HasOne);

inc_fn!(u64, U64HasOne);

//u128

inc_fn!(usize, USizeHasOne);
*/

//decrementation

/*
#[macro_export]
macro_rules! dec_fn
{

    ($value_type:ty, $has_one:ty) => //$label:ident, $
    {

        paste! {

            pub fn [<dec_ $value_type>](value: &mut $value_type) -> async_graphql::Result<$value_type>
            {

                *value -= $has_one::one();

                Ok(*value)

            }

        }

    } 

}

//incrementation

dec_fn!(f32, F32HasOne);

dec_fn!(f64, F64HasOne);

//

dec_fn!(i8, I8HasOne);

dec_fn!(i16, I16HasOne);

dec_fn!(i32, I32HasOne);

dec_fn!(i64, I64HasOne);

//i128

dec_fn!(isize, ISizeHasOne);

//

dec_fn!(u8, U8HasOne);

dec_fn!(u16, U16HasOne);

dec_fn!(u32, U32HasOne);

dec_fn!(u64, U64HasOne);

//u128

dec_fn!(usize, USizeHasOne);
*/

/*
#[macro_export]
macro_rules! inc_fl_fn
{

    ($label:ident, $value_type:ty) =>
    {

        paste! {

            pub fn [<$label _inc>](value: &mut $value_type) -> async_graphql::Result<$value_type>
            {

                *value += 1.0;

                Ok(*value)

            }

        }

    } 

}

inc_fl_fn!(f32, f32);

inc_fl_fn!(f64, f64);

#[macro_export]
macro_rules! inc_int_fn
{

    ($label:ident, $value_type:ty) =>
    {

        paste! {

            pub fn [<$label _inc>](value: &mut $value_type) -> async_graphql::Result<$value_type>
            {

                *value += 1;

                Ok(*value)

            }

        }

    } 

}

inc_int_fn!(i8, i8);

inc_int_fn!(i16, i16);

inc_int_fn!(i32, i32);

inc_int_fn!(i64, i64);

inc_int_fn!(i128, i128);

inc_int_fn!(isize, isize);

//

inc_int_fn!(u8, u8);

inc_int_fn!(u16, u16);

inc_int_fn!(u32, u32);

inc_int_fn!(u64, u64);

inc_int_fn!(u128, u128);

inc_int_fn!(usize, usize);
*/

