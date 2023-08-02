use async_graphql::*;

const UNIT: &str = "()";

pub fn get_unit_str() -> &'static str
{

    UNIT

}

#[derive(SimpleObject, Clone, Copy)] //, InputObject
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

