use async_graphql::*;

use std::mem::size_of_val;

const UNIT: &str = "()";

pub fn get_unit_value_str() -> &'static str
{

    UNIT

}

/*
async_graphql_derive
proc_macro InputObject
the trait bound `&'static str: InputType` is not satisfied
the trait `InputType` is implemented for `std::string::String`rustcE0277
the trait bound `&str: InputType` is not satisfied
the trait `InputType` is implemented for `std::string::String`rustcE0277
error.rs(185, 9): required by a bound in `InputValueError<T>`
No quick fixes available
*/

#[derive(SimpleObject, Clone, Copy, Default)] //InputObject, //, InputObject
pub struct UnitValue
{

    //#[graphql(skip)]
    value: &'static str

}

impl UnitValue
{

    pub fn new() -> Self
    {

        Self
        {

            value: UNIT

        }

    }

    pub fn get_value(&self) -> &str
    {

        self.value

    }

}

pub fn size_of_unit_value_str() -> usize
{

    size_of_val(UNIT)

}

//https://async-graphql.github.io/async-graphql/en/define_simple_object.html

/*
#[derive(ComplexObject, Clone, Copy, Default)] //SimpleObject, 
#[graphql(complex)]
pub struct UnitValue();


impl UnitValue
{

    pub fn new() -> Self
    {

        Self()

    }

}

#[ComplexObject]
impl UnitValue
{

    async fn value(&self) -> &str
    {

        UNIT

    }

}
*/
