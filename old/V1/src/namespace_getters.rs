use scc::HashMap;

use crate::{identifier::*, async_graphql_value_containers::{AnyObject, I128Scalar, U128Scalar}, all_types::Type, consts::UnitValue};

//Non-async store and namespace setters and getters

//StoreAnd

pub trait NamespaceGetters
{

    fn get_bool(&self, identifier: &Identifier) -> async_graphql::Result<bool>;

    fn get_char(&self, identifier: &Identifier) -> async_graphql::Result<char>;

    fn get_f32(&self, identifier: &Identifier) -> async_graphql::Result<f32>;

    fn get_f64(&self, identifier: &Identifier) -> async_graphql::Result<f64>;

    fn get_i8(&self, identifier: &Identifier) -> async_graphql::Result<i8>;

    fn get_i16(&self, identifier: &Identifier) -> async_graphql::Result<i16>;

    fn get_i32(&self, identifier: &Identifier) -> async_graphql::Result<i32>;

    fn get_i64(&self, identifier: &Identifier) -> async_graphql::Result<i64>;

    fn get_i128(&self, identifier: &Identifier) -> async_graphql::Result<I128Scalar>;

    fn get_isize(&self, identifier: &Identifier) -> async_graphql::Result<isize>;

    //

    fn get_u8(&self, identifier: &Identifier) -> async_graphql::Result<u8>;

    fn get_u16(&self, identifier: &Identifier) -> async_graphql::Result<u16>;

    fn get_u32(&self, identifier: &Identifier) -> async_graphql::Result<u32>;

    fn get_u64(&self, identifier: &Identifier) -> async_graphql::Result<u64>;

    fn get_u128(&self, identifier: &Identifier) -> async_graphql::Result<U128Scalar>;

    fn get_unit(&self, identifier: &Identifier) -> async_graphql::Result<UnitValue>;

    fn get_usize(&self, identifier: &Identifier) -> async_graphql::Result<usize>;
    //

    fn get_string(&self, identifier: &Identifier) -> async_graphql::Result<String>;

    fn get_identifier(&self, identifier: &Identifier) -> async_graphql::Result<Identifier>;

    fn get_value(&self, identifier: &Identifier) -> async_graphql::Result<Option<AnyObject>>;

    //

    fn get_count(&self) -> usize;

    fn get_value_names(&self) -> Vec<String>;

    fn get_value_names_and_types(&self) -> std::collections::HashMap<String, Type>;

    fn get_type(&self, identifier: &Identifier) -> async_graphql::Result<Type>;

}


