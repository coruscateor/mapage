use async_graphql::*;

use std::mem::size_of_val;

const OK: &str = "Ok";

pub fn get_ok_value_str() -> &'static str
{

    OK

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
pub struct OkValue
{

    value: &'static str

}

impl OkValue
{

    pub fn new() -> Self
    {

        Self
        {

            value: OK

        }

    }

    pub fn get_value(&self) -> &str
    {

        self.value

    }

}

pub fn size_of_ok_value_str() -> usize
{

    size_of_val(OK)

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
