use async_graphql::*;

//https://doc.rust-lang.org/std/ops/index.html

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Bop //BinaryOp
{

    Add,
    //AddAssign,
    BitAnd,
    //BitAndAssign,
    BitOr,
    BitXor,
    //BitXorAssign,
    //Deref
    //DerefMut
    Div,
    //DivAssign,
    //Drop
    //Fn
    //...
    //Index, //separate method
    //IndexMut
    Mul,
    //MulAssign,
    //...
    Rem,
    //RemAssign,
    Shl,
    //ShlAssign,
    Shr,
    //ShrAssign,
    Sub,
    //SubAssign
    
}
