use crate::{memrs_type::Type, object::Object};

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
pub struct Something
{

    contained: Box<dyn Object>,
    my_type: Type,
    //required_type: Type

}

impl Something
{

    pub fn new(required_type: Type) -> Result<Self>
    {

        if Something::is_valid_type(&required_type)
        {

            Ok(Self
            {

                contained: Type::get_default_value(required_type).unwrap(),
                my_type: Type::new("Something"),

            })

        }
        else {

            Err(Error::new("Type is invalid"))
            
        }

    }

    pub fn new_value<T>(&mut self, obj: T) -> Result<Self>
        where T: Object + 'static
    {

        //let t_ref = obj.get_type_ref();

        if Something::is_valid_type(obj.get_type_ref())
        {

            Ok(Self
            {

                contained: Box::new(obj),
                my_type: Type::new("Something"),
                //ob

            })

        }
        else {

            Err(Error::new("Type of passed value is invalid"))
            
        }

    }

    fn is_valid_type(typ: &Type) -> bool
    {

        let nm = typ.get_name();

        nm != "Whatever" && nm != "Optional" && nm != "Nothing" //&& nm != "Required" //&& nm != "Something"

    } 

    fn get_contained_type(&self) -> Type
    {
        
        self.contained.get_type()

    }

    fn get_contained_type_ref(&self) -> &Type
    {

        self.contained.get_type_ref()

    }

    fn get_required_type(&self) -> Type
    {
        
        self.contained.get_type()

    }

    fn get_required_type_ref(&self) -> &Type
    {

        self.contained.get_type_ref()

    }

    fn set_contained<T>(&mut self, obj: T) -> Result<()>
        where T: Object + 'static
    {

        //if obj.get_type_ref().eq(self.contained.get_type_ref())
        if obj.get_type_ref() == self.contained.get_type_ref()
        {

            //let ctd = self.contained;
            
            self.contained = Box::new(obj);

            return Ok(());

        }

        Err(Error::new("Passed type not the same as the required type"))
       
    }

}

impl Object for Required
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