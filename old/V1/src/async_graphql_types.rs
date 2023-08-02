use async_graphql::*;

//InputObject, SimpleObject, 

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum NonGenericType
{

    Bool,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Unit,
    Usize,
    String,
    //Nothing

}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum GenericType
{

    //Optional,
    //Required,
    Array,
    Tuple,
    Option,
    Vec,
    BTreeSet,
    BinaryHeap,
    HashMap,    //2/3
    HashSet,    //1/2
    LinkedList,
    VecDeque,   //1/2
    Whatever,
    List,       //corlib
    Dictionary

}

