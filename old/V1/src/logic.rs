use async_graphql::*;

#[derive(InputObject, SimpleObject)]
pub struct Bool
{

    pub value: bool

}

impl Bool
{
    
    pub fn new(value: bool) -> Self
    {

        Self
        {

            value

        }

    }

}
