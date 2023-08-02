//use std::sync::Arc;

//use crate::generic_type_info::*;

use std::{rc::Rc, sync::Arc, collections::HashMap, array};

use async_graphql::{Enum, SimpleObject};

///
/// Rcd (Reference Counted) is an Arc in this build
/// 
pub type Rcd<T> = std::sync::Arc<T>;

pub type GPSize = Option<usize>; //Generic Parameters Size

//use std::sync::Mutex;

///Mutex
pub type Mtx<T> = std::sync::Mutex<T>;

pub type MtxGuard<'a, T> = std::sync::MutexGuard<'a, T>;

///Rwlock
pub type Rwlk<T> = std::sync::RwLock<T>;

//Type names

pub const BOOL: &str = "bool";

pub const CHAR: &str = "char";

pub const F32: &str = "f32";

pub const F64: &str = "f64";

pub const I8: &str = "i8";

pub const I16: &str = "i16";

pub const I32: &str = "i32";

pub const I64: &str = "i64";

pub const I128: &str = "i128";

pub const ISIZE: &str = "isize";

pub const U8: &str = "u8";

pub const U16: &str = "u16";

pub const U32: &str = "u32";

pub const U64: &str = "u64";

pub const U128: &str = "u128";

pub const UNIT: &str = "unit";

pub const USIZE: &str = "usize";

//

pub const NOTHING: &str = "Nothing";

//pub const BOOL: &str = "Optional";

//pub const BOOL: &str = "Required";

//pub const BOOL: &str = "Something";

pub const WHATEVER: &str = "Whatever";

/*
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Crate
{

    std,
    memrs,
    corlib,
    scc

}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TypeName
{

    //No crate

    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    unit,
    usize,

    //Non-Generics

    //std

    String,

    //Generics

    //memrs

    Whatever,

    //std

    Option,

    //memrs - generic adaption for memrs

    Array,
    Tuple,

    //std

    Vec,
    BTreeSet,
    BinaryHeap,

    //std and scc

    HashMap,    //2/3 //String key only
    HashSet,    //1/2

    //std

    LinkedList,
    VecDeque,   //1/2

    //corlib

    List,       
    Dictionary,

}
*/

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DiscreteTypeName
{

    //No crate

    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    unit,
    usize,

    //Non-Generics

    //std

    std_String,

    memrs_Identifier,

    memrs_Whatever,

    //Generics

    //memrs

    //std

    std_Option,

    //memrs - generic adaption for memrs

    memrs_Array,
    memrs_Tuple,

    //std

    std_Vec,
    std_BTreeSet,
    std_BinaryHeap,

    //std and scc

    std_HashMap,    //2/3 //String key only
    std_HashSet,    //1/2

    scc_HashMap,
    scc_HashSet,

    //std

    std_LinkedList,
    std_VecDeque,   //1/2

    //corlib

    corlib_List,       
    corlib_Dictionary,

}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TraitImpl
{

    PartailEq,
    Eq

}

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct Type
{

    //crate_name: Option<Crate>,
    //name: TypeName, //&'static str,
    discrete_type_name: DiscreteTypeName,
    generic_type_spec: Option<GenericTypeSpec>,
    generic_params: Option<Vec<Type>> //Option<Arc<Vec<Type>>>,
    //spec: TypeSpec
    //is_generic: bool,


}

impl Type
{

    
    pub fn new(discrete_type_name: DiscreteTypeName, generic_type_spec: Option<GenericTypeSpec>, generic_params: Option<Vec<Type>>) -> Self //Option<Arc<Vec<Type>>>) -> Self
    {

        Self
        {

            discrete_type_name,
            generic_type_spec,
            generic_params

        }

    }

    pub fn get_discrete_type_name(&self) -> DiscreteTypeName
    {

        self.discrete_type_name

    }

    pub fn get_generic_type_spec(&self) -> &Option<GenericTypeSpec>
    {

        &self.generic_type_spec

    }

    pub fn get_generic_params(&self) -> &Option<Vec<Type>> //Option<Arc<Vec<Type>>>
    {

        &self.generic_params

    }

}

//pub type GenericRestrictions = Option<HashMap<u8, Vec<TraitImpl>>>;

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct GenericTypeSpec
{

    required_params: Option<usize>,
    //restrictions
    restrictions: Option<Vec<Option<TraitImpl>>> //Option<HashMap<u8, Vec<TraitImpl>>> //GenericRestrictions //Option<Arc<HashMap<u8, Vec<TraitImpl>>>> //Vec<Option<Vec<TraitImpl>>>>>,

}

impl GenericTypeSpec
{

    pub fn new(required_params: Option<usize>, restrictions:  Option<Vec<Option<TraitImpl>>>) -> Self //Option<HashMap<u8, Vec<TraitImpl>>>) -> Self //Option<Arc<Vec<Option<Vec<TraitImpl>>>>>) -> Self
    {

        Self
        {

            required_params,
            restrictions

        }

    }

    pub fn get_required_params(&self) -> Option<usize>
    {

        self.required_params

    }
    
    pub fn get_restrictions(&self) -> &Option<Vec<Option<TraitImpl>>> //&Option<HashMap<u8, Vec<TraitImpl>>> //&Option<Arc<Vec<Option<Vec<TraitImpl>>>>>
    {

        &self.restrictions

    }

}

pub struct AllTypes();

impl AllTypes
{

    pub fn get_bool_type() -> Type
    {

        Type::new(DiscreteTypeName::bool, None, None)

    }

    pub fn get_char_type() -> Type
    {

        Type::new(DiscreteTypeName::char, None, None)
        
    }

    pub fn get_f32_type() -> Type
    {

        Type::new(DiscreteTypeName::f32, None, None)
        
    }

    pub fn get_f64_type() -> Type
    {

        Type::new(DiscreteTypeName::f64, None, None)
        
    }

    pub fn get_i8_type() -> Type
    {

        Type::new(DiscreteTypeName::i8, None, None)
        
    }

    pub fn get_i16_type() -> Type
    {

        Type::new(DiscreteTypeName::i16, None, None)
        
    }

    pub fn get_i32_type() -> Type
    {

        Type::new(DiscreteTypeName::i32, None, None)
        
    }

    pub fn get_i64_type() -> Type
    {

        Type::new(DiscreteTypeName::i64, None, None)
        
    }

    pub fn get_i128_type() -> Type
    {

        Type::new(DiscreteTypeName::i128, None, None)
        
    }

    pub fn get_isize_type() -> Type
    {

        Type::new(DiscreteTypeName::isize, None, None)
        
    }

    pub fn get_u8_type() -> Type
    {

        Type::new(DiscreteTypeName::u8, None, None)
        
    }

    pub fn get_u16_type() -> Type
    {

        Type::new(DiscreteTypeName::u16, None, None)
        
    }

    pub fn get_u32_type() -> Type
    {

        Type::new(DiscreteTypeName::u32, None, None)
        
    }

    pub fn get_u64_type() -> Type
    {

        Type::new(DiscreteTypeName::u64, None, None)
        
    }

    pub fn get_u128_type() -> Type
    {

        Type::new(DiscreteTypeName::u128, None, None)
        
    }

    pub fn get_unit_type() -> Type
    {

        Type::new(DiscreteTypeName::unit, None, None)
        
    }


    pub fn get_usize_type() -> Type
    {

        Type::new(DiscreteTypeName::usize, None, None)
        
    }

    //

    pub fn get_string_type() -> Type
    {

        Type::new(DiscreteTypeName::std_String, None, None)
        
    }

    pub fn get_whatever_type() -> Type
    {

        Type::new(DiscreteTypeName::memrs_Whatever,  None, None) //Some(GenericTypeSpec::new(Some(1), None)), None)
        
    }

    /*
    pub fn get_whatever_type(type_param: Type) -> Type
    {

        //let param = Vec::with_capacity(1);

        //param

        //Whatever: a dynamic generic type

        Type::new(DiscreteTypeName::memrs_Whatever, Some(GenericTypeSpec::new(Some(1), None)), Some(vec![type_param])) //Some(GenericTypeSpec::new(Some(1), None)), None)
        
    }

    pub fn get_whatever_type_template() -> Type
    {

        //Whatever: a dynamic generic type

        Type::new(DiscreteTypeName::memrs_Whatever, Some(GenericTypeSpec::new(Some(1), None)), None) //Some(GenericTypeSpec::new(Some(1), None)), None)
        
    }
    */

    pub fn get_option_type(type_param: Type) -> Type
    {

        Type::new(DiscreteTypeName::std_Option, Some(GenericTypeSpec::new(Some(1), None)), Some(vec![type_param])) //Some(GenericTypeSpec::new(Some(1), None)), None)
        
    }

    pub fn get_option_type_template() -> Type
    {

        //Whatever: a dynamic generic type

        Type::new(DiscreteTypeName::std_Option, Some(GenericTypeSpec::new(Some(1), None)), None) //Some(GenericTypeSpec::new(Some(1), None)), None)
        
    }


}

/*
#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct TypeSpec
{

    is_generic: bool,
    required_generic_params: Option<usize>,

}
*/

/*
#[derive(Clone, PartialEq, Eq)]
pub enum TypeInitInfo
{

    /*
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    unit,
    usize,
    Nothing,
    */

    NonGeneric,

    //Generics

    Whatever{params_count: GPSize},
    Option{params_count: GPSize},
    Array{params_count: GPSize},
    Tuple{params_count: GPSize},
    Vec{params_count: GPSize},
    BTreeSet{params_count: GPSize},
    BinaryHeap{params_count: GPSize},
    HashMap{params_count: GPSize},    //2/3 //String key only
    HashSet{params_count: GPSize},    //1/2
    LinkedList{params_count: GPSize},
    VecDeque{params_count: GPSize},   //1/2
    List{params_count: GPSize},       //corlib
    Dictionary{params_count: GPSize},

}

impl TypeInitInfo
{

    pub fn new_whatever() -> Self
    {

        Self::Whatever
        { 
            
            params_count: Some(1)
        
        }

    }

    pub fn new_option() -> Self
    {

        Self::Option
        { 
            
            params_count: Some(1)
        
        }

    }

    pub fn new_array() -> Self
    {

        Self::Array
        { 
            
            params_count: Some(1)
        
        }

    }

    pub fn new_tuple() -> Self
    {

        Self::Tuple
        { 
            
            params_count: None
        
        }

    }

}

/*
memrs::all_types
pub enum TypeInstance
0 implementations

recursive type `TypeInstance` has infinite size
recursive type has infinite sizerustcE0072
all_types.rs(178, 21): recursive without indirection
all_types.rs(179, 20): recursive without indirection
all_types.rs(178, 21): insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `TypeInstance` representable: `Box<`, `>`, `Box<`, `>`
No quick fixes available
*/

#[derive(Clone, PartialEq, Eq)]
pub enum Type //Instance //Generic
{

    //Non-Generics

    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    unit,
    usize,
    String,
    //Nothing,

    //Generics

    Array{params: Rcd<Vec<Type>>},
    Tuple{params: Rcd<Vec<Type>>},
    Option{param: Rcd<Type>},
    Vec{params: Rcd<Type>},
    BTreeSet{params: Rcd<Vec<Type>>},
    BinaryHeap{params: Rcd<Vec<Type>>},
    HashMap{params: Rcd<Vec<Type>>},    //2/3 //String key only
    HashSet{params: Rcd<Vec<Type>>},    //1/2
    LinkedList{params: Rcd<Vec<Type>>},
    VecDeque{params: Rcd<Vec<Type>>},   //1/2
    Whatever{param: Rcd<Type>},
    List{params: Rcd<Type>},       //corlib
    Dictionary{params: Rcd<Vec<Type>>},

}

impl Type
{

    pub fn new_whatever(param: &Rcd<Type>) -> Self
    {

        Self::Whatever
        {

            param: param.clone()
        
        }

    }

    pub fn new_option(param: &Rcd<Type>) -> Self
    {

        Self::Option
        { 
            
            param: param.clone()
        
        }

    }

    pub fn new_array(params: &Rcd<Vec<Type>>) -> Self
    {

        Self::Array
        { 
            
            params: params.clone()
        
        }

    }

    pub fn new_tuple(params: &Rcd<Vec<Type>>) -> Self
    {

        Self::Tuple
        { 
            
            params: params.clone()
        
        }

    }

}

pub struct AllTypes
{

    //Non-generics

    bool_type: Rcd<Type>,
    char_type: Rcd<Type>,
    f32_type: Rcd<Type>,
    f64_type: Rcd<Type>,
    i8_type: Rcd<Type>,
    i16_type: Rcd<Type>,
    i32_type: Rcd<Type>,
    i64_type: Rcd<Type>,
    i128_type: Rcd<Type>,
    isize_type: Rcd<Type>,
    u8_type: Rcd<Type>,
    u16_type: Rcd<Type>,
    u32_type: Rcd<Type>,
    u64_type: Rcd<Type>,
    u128_type: Rcd<Type>,
    unit_type: Rcd<Type>,
    usize_type: Rcd<Type>,
    String_type: Rcd<Type>,
    //Nothing_type: Rcd<Type>,

    //Generics

}

impl AllTypes
{

    pub fn new() -> Self
    {

        Self
        {

            //Scalars

            bool_type: Rcd::new(Type::bool),
            char_type: Rcd::new(Type::char),
            f32_type: Rcd::new(Type::f32),
            f64_type: Rcd::new(Type::f64),
            i8_type: Rcd::new(Type::i8),
            i16_type: Rcd::new(Type::i16),
            i32_type: Rcd::new(Type::i32),
            i64_type: Rcd::new(Type::i64),
            i128_type: Rcd::new(Type::i128),
            isize_type: Rcd::new(Type::isize),
            u8_type: Rcd::new(Type::u8),
            u16_type: Rcd::new(Type::u16),
            u32_type: Rcd::new(Type::u32),
            u64_type: Rcd::new(Type::u64),
            u128_type: Rcd::new(Type::u128),
            unit_type: Rcd::new(Type::unit),
            usize_type: Rcd::new(Type::usize),
            String_type: Rcd::new(Type::String),
            //Nothing_type: Rcd::new(Type::Nothing),

            //Vectors

        }

    }

    //Scalars

    pub fn get_bool_type(&self) -> Rcd<Type>
    {

        self.bool_type.clone()

    }

    pub fn get_char_type(&self) -> Rcd<Type>
    {

        self.char_type.clone()

    }

    pub fn get_f32_type(&self) -> Rcd<Type>
    {

        self.f32_type.clone()

    }

    pub fn get_f64_type(&self) -> Rcd<Type>
    {

        self.f64_type.clone()

    }

    pub fn get_i8_type(&self) -> Rcd<Type>
    {

        self.i8_type.clone()

    }

    pub fn get_i16_type(&self) -> Rcd<Type>
    {

        self.i16_type.clone()

    }

    pub fn get_i32_type(&self) -> Rcd<Type>
    {

        self.i32_type.clone()

    }

    pub fn get_i64_type(&self) -> Rcd<Type>
    {

        self.i64_type.clone()

    }

    pub fn get_i128_type(&self) -> Rcd<Type>
    {

        self.i128_type.clone()

    }

    pub fn get_isize_type(&self) -> Rcd<Type>
    {

        self.isize_type.clone()

    }

    pub fn get_u8_type(&self) -> Rcd<Type>
    {

        self.u8_type.clone()

    }

    pub fn get_u16_type(&self) -> Rcd<Type>
    {

        self.u16_type.clone()

    }

    pub fn get_u32_type(&self) -> Rcd<Type>
    {

        self.u32_type.clone()

    }

    pub fn get_u64_type(&self) -> Rcd<Type>
    {

        self.u64_type.clone()

    }

    pub fn get_u128_type(&self) -> Rcd<Type>
    {

        self.usize_type.clone()

    }

    pub fn get_unit_type(&self) -> Rcd<Type>
    {

        self.unit_type.clone()

    }

    pub fn get_usize_type(&self) -> Rcd<Type>
    {

        self.usize_type.clone()

    }

    pub fn get_string_type(&self) -> Rcd<Type>
    {

        self.String_type.clone()

    }
    
    /*
    pub fn get_nothing_type(&self) -> Rcd<Type>
    {

        self.Nothing_type.clone()

    }
    */

    //Store Types

    pub fn get_rcd_type(&self, the_type: &Type) -> Rcd<Type>
    {

        match the_type
        {

            Type::bool =>
            {

                self.get_bool_type()

            }
            Type::char =>
            {

                self.get_char_type()

            }
            Type::f32 =>
            {

                self.get_f32_type()

            }
            Type::f64 =>
            {

                self.get_f64_type()

            }
            Type::i8 =>
            {

                self.get_i8_type()

            }
            Type::i16 =>
            {

                self.get_i16_type()

            }
            Type::i32 =>
            {

                self.get_i32_type()

            }
            Type::i64 =>
            {

                self.get_i64_type()

            }
            Type::i128 =>
            {

                self.get_i32_type()

            }
            Type::isize =>
            {

                self.get_isize_type()

            }
            Type::u8 =>
            {

                self.get_u8_type()

            }
            Type::u16 =>
            {

                self.get_u16_type()

            }
            Type::u32 =>
            {

                self.get_u32_type()

            }
            Type::u64 =>
            {

                self.get_u64_type()

            }
            Type::u128 =>
            {

                self.get_u128_type()

            }
            Type::unit =>
            {

                self.get_unit_type()

            }
            Type::usize =>
            {

                self.get_usize_type()

            }
            Type::String =>
            {

                self.get_string_type()

            }
            /*
            Type::Nothing =>
            {

                self.get_nothing_type()

            }
            */
            _ =>
            {

                panic!("not supported yet")

            }
            
        }

    }

    //Vectors

    //



}
*/

/*
static mut ALLTYPES: AllTypes = AllTypes::new();

pub fn get_all_types() -> &'static AllTypes
{

    unsafe {
     
        &ALLTYPES
        
    }

}
*/

