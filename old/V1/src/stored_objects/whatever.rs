use crate::{stored_object::StoredObject};

//memrs_type::*,, non_generic_type_controller::*

//use super::nothing::Nothing;

use async_graphql::Result;

use crate::uniary_operations::*;

use crate::binary_operations::*;

//use crate::ternary_operations::*;

use crate::async_graphql_value_containers::*;

use crate::errors::*;

use crate::all_types::*;

use crate::async_graphql_generic_type_info::*;

use super::stored_unit::*;

use crate::identifier::*;

//static WHATEVER_TYPE: Type = Type::new("Whatever");

/*

cannot call non-const fn `memrs_type::Type::new` in statics
calls in statics are limited to constant functions, tuple structs and tuple variantsrustcE0015

 */

///
///Indicates that type doesn't matter, can be used a placeholder
///
//pub struct Whatever();

//Immutable

pub trait StoredWhatever
{

    fn get_contained_type(&self) -> Type; //, all_types: &AllTypes
    /*{
        
        self.contained.get_type()

    }*/

    fn set_contained<T>(&mut self, obj: T)  -> Result<()>
        where T: StoredObject + 'static
    {

        invalid_operation_instance_is_immuatable()

    }

}

///Contains/points to a value of any other type.
///Not a generic, more like a null-pointer. 
pub struct Whatever
{

    contained: Box<dyn StoredObject>,
    //my_type: Type

}

impl Whatever
{

    pub fn new() -> Self
    {

        Self
        {

            contained: Box::new(StoredUnit::new()),
            //my_type: Type::new_whatever(&get_all_types().get_nothing_type()) //get_STC().get_whatever_type() //Type::new("Whatever")

        }

    }

    /*
    fn get_contained_type(&self) -> Type
    {
        
        self.contained.get_type()

    }

    /*
    fn get_contained_type_ref(&self) -> &Type
    {

        &self.contained.get_type_ref()

    }
    */

    fn set_contained<T>(&mut self, obj: T)  -> Result<()>
        where T: StoredObject + 'static
    {
        
        //self.contained = Box::new(obj);

        //Ok(())

        invalid_operation_instance_is_immuatable()

    }
    */

}

impl StoredWhatever for Whatever
{

    fn get_contained_type(&self) -> Type //, all_types: &AllTypes
    {
        
        self.contained.get_type() //all_types)

    }

}

impl StoredObject for Whatever
{

    fn get_type(&self) -> Type //, all_types: &AllTypes
    {
        
        //Type::new("Whatever")

        //WHATEVER_TYPE.clone()

        //self.my_type //.clone()

        //let the_type = self.contained.get_type(&all_types);

        //Type::new_whatever(&all_types.get_rcd_type(&the_type))  //&get_all_types().get_rcd_type(&self.my_type))

        //AllTypes::get_whatever_type(self.contained.get_type()) //all_types

        AllTypes::get_whatever_type()

    }

    fn get_value(&self) -> async_graphql::Result<Option<AnyObject>>
    {

        self.contained.get_value()

        /*
        if let Some(val) = self.contained
        {

            val.get_value()

        }
        else
        {

            Ok(None)

        }
        */

    }
    
    fn get_bool(&self) -> async_graphql::Result<bool>
    {

        self.contained.get_bool()
        
    }

    /*
    fn get_type_ref(&self) -> &Type
    {
        
        //&WHATEVER_TYPE

        &self.my_type

    }
    */

    /*
    fn uop(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        self.contained.uop(op)
        
    }
    */

    /*
    fn uop_mut(&self, op: Uop) -> async_graphql::Result<NumericOrBool>
    {
        
        //self.contained.uop(op)

        invalid_operation_instance_is_immuatable()

    }
    */

    /*
    fn bop(&self, op: Bop, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>
    {
        
        self.contained.bop(op, right_side)

    }
    */

    /*
    fn bop_mut(&self, op: Bop, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>
    {
        
        //self.contained.bop(op, right_side)
        
        invalid_operation_instance_is_immuatable()

    }
    */

    /*
    fn top(&self, op: Top, middle: NumericOrKeyNSOrBool, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool>
    {
        
        self.contained.top(op, middle, right_side)
        
    }
    */

    /*
    fn top_mut(&self, op: Top, middle: NumericOrKeyNSOrBool, right_side: NumericOrKeyNSOrBool) -> async_graphql::Result<NumericOrBool> {
        todo!()
    }
    */

    /* 
    fn is_mutable(&self) -> bool
    {

        false

    }
    */

}