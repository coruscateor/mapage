//use crate::memrs_type::*;

use crate::all_types::*;

pub struct Value<T>
{

    value: T

}

impl<T> Value<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            value

        }

    }

}

//

/*
pub struct TypedValue<T>
{

    value: T,
    my_type: Type

}

impl<T> TypedValue<T>
{

    pub fn typed_new(value: T, my_type: Type) -> Self
    {

        Self
        {

            value,
            my_type

        }

    }

    pub fn get_contained_type(&self) -> Type
    {

        self.my_type.clone()

    }

    /*
    pub fn value_ref(&self) -> &T
    {

        &self.value

    }

    pub fn type_ref(&self) -> &Type
    {

        &self.my_type

    }

    pub fn type_clone(&self) -> Type
    {

        self.my_type.clone()

    }
    */

}
*/

//
// Obviously the memrs value is not known at compile time stored value not known at compile time
//

/*

expected item after doc comment
this doc comment doesn't document anythingrustc
value.rs(76, 1): other attributes here

*/

/*
pub struct GenericValue<T>
{

    value: T,
    my_type: Type,
    generic_type: Type

}

impl<T> GenericValue<T>
{

    pub fn generic_new(value: T, my_type: Type) -> Self
    {

        Self
        {

            value,
            my_type

        }

    }

    pub fn value_ref(&self) -> &T
    {

        &self.value

    }

    pub fn type_ref(&self) -> &Type
    {

        &self.my_type

    }

    pub fn type_clone(&self) -> Type
    {

        self.my_type.clone()

    }

}
*/
