

//Namespaces

use crate::{uniary_operations::Uop, errors::invalid_operation, async_graphql_value_containers::NumericOrBool, binary_operations::Bop};

use async_trait::async_trait;

pub trait Storage
{

    fn uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        invalid_operation()

    }

    fn bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> //NumericOrKeyNSOrBool
    {

        invalid_operation()

    }

    fn bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        invalid_operation()

    }

}

#[async_trait]
pub trait AsyncStorage : Storage
{

    async fn async_uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {

        self.uop(op)

    }

    async fn async_bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool> //NumericOrKeyNSOrBool
    {

        self.bop(op, right_side)

    }

    async fn async_bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.bop_self(op)

    }

}

