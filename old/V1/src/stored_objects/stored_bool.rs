use super::stored_unit::*;

use crate::async_graphql_value_containers::AnyObject;
use crate::consts::UnitValue;
use crate::identifier::*;

use crate::
{
    
    stored_object::StoredObject, 
    //value::*,
    all_types::*,
    errors::invalid_operation_instance_is_immuatable,
    uniary_operations::*, async_graphql_value_containers::{NumericOrBool, NumericOrIdentifierOrBool, Numeric, BoolValue}, binary_operations::Bop,
    errors::*,
    //logic::Bool

};

use std::cell::Cell;

//memrs_type::Type,

//static BOOL_TYPE: Type = Type::new("bool");

//pub struct Bool 

//pub type Bool = Value<bool>;

#[derive(Default)]
pub struct StoredBool
{

    //my_type: Type
    value: bool

}

impl StoredBool
{
    
    pub fn new(value: bool) -> Self
    {

        Self
        {

            value

        }

        //TypedValue::typed_new(value, Type::new("bool"))

        /*
        Self
        {
            value,
            my_type: Type::new("bool")

        }
        */

    }

    pub fn new_default() -> Self
    {

        Self
        {

            value: Default::default()

        }

    }

}

impl StoredObject for StoredBool
{

    fn get_type(&self) -> Type //, _all_types: &AllTypes
    {
        
        //BOOL_TYPE.clone()

        //self.get_type()

        //Type::bool

        AllTypes::get_bool_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        //self.value.get_value(identifier)
        
        Ok(Some(AnyObject::Bool(BoolValue::new(self.value))))

    }

    fn uop(&mut self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        if let Uop::Not = op
        {

            self.value = !self.value;

            Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

        }
        else
        {
            
            invalid_operation()

        }

    }

    fn bop(&mut self, op: Bop, right_side: NumericOrBool) -> async_graphql::Result<NumericOrBool>
    {

        let result = get_bool(right_side);

        let res_right_side;

        match result
        {

            Ok(res) =>
            {

                res_right_side = res;

            }
            Err(err) =>
            {

                return Err(err);

            }
            
        }

        let value = self.value;

        match op
        {

            Bop::BitAnd =>
            {

                self.value = value & res_right_side;

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            Bop::BitOr =>
            {

                self.value = value | res_right_side;

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            Bop::BitXor =>
            {

                self.value = value ^ res_right_side;

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            _ =>
            {

                invalid_operation()

            }

        }
        
    }

    fn bop_self(&mut self, op: Bop) -> async_graphql::Result<NumericOrBool>
    {

        let value = self.value;
        
        match op
        {

            Bop::BitAnd =>
            {

                self.value = value & value;

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            Bop::BitOr =>
            {

                self.value = value | value;

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            Bop::BitXor =>
            {

                self.value = (value ^ value);

                Ok(NumericOrBool::Bool(BoolValue::new(self.value)))

            }
            _ =>
            {

                invalid_operation()

            }

        }

    }

    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        //Ok(Bool::default())

        Ok(self.value)
        
    }

    fn set_bool(&mut self, input: bool) -> async_graphql::Result<UnitValue>
    {

        self.value = input;

        Ok(UnitValue::new())

    }
    
}

fn get_bool(right_side: NumericOrBool) -> async_graphql::Result<bool> //async_graphql::Result<bool>
{

    //let res: bool;

    match right_side
    {

        NumericOrBool::Bool(item) =>
        {

            //return Ok(item.value);

            Ok(item.value)

        }
        NumericOrBool::Numeric(numeric_item) =>
        {

            match numeric_item
            {

                Numeric::F32(item) =>
                {

                    //let res = item.value.try_into().map_err(op);

                    //item.value as bool

                    Ok(item.value != 0.0)

                }
                Numeric::F64(item) =>
                {

                    Ok(item.value != 0.0)

                }
                Numeric::I8(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::I16(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::I32(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::I64(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::I128(item) =>
                {

                    //Ok(item.value.parse::<i128>()? != 0)

                    Ok(item.value.0 != 0)

                }
                Numeric::Isize(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::U8(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::U16(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::U32(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::U64(item) =>
                {

                    Ok(item.value != 0)

                }
                Numeric::U128(item) =>
                {

                    //Ok(item.value.parse::<u128>()? != 0)

                    Ok(item.value.0 != 0)

                }
                Numeric::Usize(item) =>
                {

                    Ok(item.value != 0)

                }

            }

        }

    }

}
