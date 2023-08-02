use crate::{identifier::*, uniary_operations::Uop, async_graphql_value_containers::{NumericOrBool, I128Scalar, U128Scalar, AnyObject, AnyInputObject}, binary_operations::Bop, consts::UnitValue, async_graphql_types::{NonGenericType, GenericType}};

//StoreAnd

pub trait NamespaceSettersMut
{

    fn create_stored_object(&mut self, identifier: &Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>;

    fn create_stored_object_mut(&mut self, identifier: &Identifier, the_type: NonGenericType) -> async_graphql::Result<UnitValue>;

    fn create_generic_stored_object(&mut self, identifier: &Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>;

    fn create_generic_stored_object_mut(&mut self, identifier: &Identifier, the_type: GenericType) -> async_graphql::Result<UnitValue>;

    //

    fn uop(&mut self, identifier: &Identifier, op: Uop) -> async_graphql::Result<NumericOrBool>;

    fn bop(&mut self, identifier: &Identifier, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool>;

    fn bop_self(&mut self, identifier: &Identifier, op: Bop) -> async_graphql::Result<NumericOrBool>;

    /*
    fn is_mutable(&mut self) -> bool
    {

        true

    }
    */

    fn set_bool(&mut self, identifier: &Identifier, input: bool) -> async_graphql::Result<UnitValue>;

    fn set_char(&mut self, identifier: &Identifier, input: char) -> async_graphql::Result<UnitValue>;

    fn set_f32(&mut self, identifier: &Identifier, input: f32) -> async_graphql::Result<UnitValue>;

    fn set_f64(&mut self, identifier: &Identifier, input: f64) -> async_graphql::Result<UnitValue>;

    fn set_i8(&mut self, identifier: &Identifier, input: i8) -> async_graphql::Result<UnitValue>;

    fn set_i16(&mut self, identifier: &Identifier, input: i16) -> async_graphql::Result<UnitValue>;

    fn set_i32(&mut self, identifier: &Identifier, input: i32) -> async_graphql::Result<UnitValue>;

    fn set_i64(&mut self, identifier: &Identifier, input: i64) -> async_graphql::Result<UnitValue>;

    fn set_i128(&mut self, identifier: &Identifier, input: I128Scalar) -> async_graphql::Result<UnitValue>;

    fn set_isize(&mut self, identifier: &Identifier, input: isize) -> async_graphql::Result<UnitValue>;

    //

    fn set_u8(&mut self, identifier: &Identifier, input: u8) -> async_graphql::Result<UnitValue>;

    fn set_u16(&mut self, identifier: &Identifier, input: u16) -> async_graphql::Result<UnitValue>;

    fn set_u32(&mut self, identifier: &Identifier, input: u32) -> async_graphql::Result<UnitValue>;

    fn set_u64(&mut self, identifier: &Identifier, input: u64) -> async_graphql::Result<UnitValue>;

    fn set_u128(&mut self, identifier: &Identifier, input: U128Scalar) -> async_graphql::Result<UnitValue>;

    fn set_unit(&mut self, identifier: &Identifier, input: UnitValue) -> async_graphql::Result<UnitValue>;

    fn set_usize(&mut self, identifier: &Identifier, input: usize) -> async_graphql::Result<UnitValue>;

    //Non=Primitive Types

    fn set_string(&mut self, identifier: &Identifier, input: String) -> async_graphql::Result<UnitValue>;

    fn set_identifier(&mut self, identifier: &Identifier, input: Identifier) -> async_graphql::Result<UnitValue>;

    fn set_value(&mut self, identifier: &Identifier, input: AnyInputObject) -> async_graphql::Result<UnitValue>;

}



