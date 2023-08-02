use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Uop //UniaryOp
{

    Inc, //rement,
    Dec, //rement,
    Neg,
    Not

}

