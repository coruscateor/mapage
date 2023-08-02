use crate::
{

    stored_object::StoredObject,
    all_types::*, errors::invalid_operation_instance_is_immuatable

};


//memrs_type::*,

//use crate::

//static WHATEVER_TYPE: Type = Type::new("Whatever");

/*

cannot call non-const fn `memrs_type::Type::new` in statics
calls in statics are limited to constant functions, tuple structs and tuple variantsrustcE0015

 */

///
///Nothing
///
//pub struct Nothing();

#[derive(Default)]
pub struct Nothing
{

    //my_type: RcdType

}

impl Nothing
{

    pub fn new() -> Self
    {

        Self
        {

            //my_type: Type::new("Nothing")

        }

    }

}

impl StoredObject for Nothing
{

    fn get_type(&self, all_types: &AllTypes) -> Type 
    {
        
        //Type::new("Whatever")

        //WHATEVER_TYPE.clone()

        //self.my_type.clone()

        Type::Nothing

    }

    /*

    fn uop(&mut self, op: crate::uniary_operations::Uop) -> async_graphql::Result<crate::numerics::NumericOrBool> {
        
        invalid_operation_instance_is_immuatable()

    }

    fn bop(&mut self, op: crate::binary_operations::Bop, right_side: crate::numerics::NumericOrBool) -> async_graphql::Result<crate::numerics::NumericOrBool> {
        
        invalid_operation_instance_is_immuatable()

    }

    fn is_mutable(&self) -> bool
    {

        false

    }
    */

    /*
    fn get_type_ref(&self) -> &Type {
        
        //&WHATEVER_TYPE

        &self.my_type

    }
    */

}