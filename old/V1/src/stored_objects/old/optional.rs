use crate::{memrs_type::Type, stored_object::Object};

use super::nothing::Nothing;

use async_graphql::{Result, Error};

//use std::ops::

//static WHATEVER_TYPE: Type = Type::new("Whatever");

/*

cannot call non-const fn `memrs_type::Type::new` in statics
calls in statics are limited to constant functions, tuple structs and tuple variantsrustcE0015

 */

///
///Indicates that type doesn't matter, can be used a placeholder
///
//pub struct Whatever();

pub struct Optional
{

    contained: Box<dyn Object>,
    my_type: Type,
    required_type: Type

}

impl Optional
{

    pub fn new(required_type: Type) -> Self
    {

        Self
        {

            contained: Box::new(Nothing::new()),
            my_type: Type::new("Optional"),
            required_type

        }

    }

    fn get_contained_type(&self) -> Type
    {
        
        self.contained.get_type()

    }

    fn get_contained_type_ref(&self) -> &Type
    {

        &self.contained.get_type_ref()

    }

    fn get_required_type(&self) -> Type
    {
        
        self.required_type.clone()

    }

    fn get_required_type_ref(&self) -> &Type
    {

        &self.required_type

    }

    fn set_contained<T>(&mut self, obj: T) -> Result<()>
        where T: Object + 'static
    {

        if obj.get_type_ref().eq(&self.required_type)
        {

            //let ctd = self.contained;
            
            self.contained = Box::new(obj);

            return Ok(());

        }

        Err(Error::new("Passed type not the same as the required type"))
       
    }

}

impl Object for Optional
{

    fn get_type(&self) -> Type {
        
        //Type::new("Whatever")

        //WHATEVER_TYPE.clone()

        self.my_type.clone()

    }

    fn get_type_ref(&self) -> &Type {
        
        //&WHATEVER_TYPE

        &self.my_type

    }



}