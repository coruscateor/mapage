use crate::{all_types::{AllTypes, Rcd}, uniary_operations::Uop, binary_operations::Bop, async_graphql_value_containers::NumericOrBool};

//StorageContainer

#[derive(Default)]
pub enum ActorMessage //<'a>
{

    //GetType{all_types: &'a AllTypes},
    GetType{all_types: Rcd<AllTypes>},
    Op{op: Uop},
    Bop{op: Bop, right_side: NumericOrBool},
    Bop_Self{op: Bop},
    IsMutable,
    #[default]
    None

}


