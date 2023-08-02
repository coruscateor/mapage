//use crate::memrs_type::*;

use crate::all_types::*;

#[derive(PartialEq, Eq, Clone)]
pub struct GenericTypeInfo
{

    name: &'static str,
    number_of_parameters: Option<usize> //,
    //is_variadic: bool

}

impl GenericTypeInfo
{

    pub fn new(name: &'static str, number_of_parameters: Option<usize>) -> Self
    {

        Self
        {

            name,
            number_of_parameters

        }

    }

    pub fn get_name(&self) -> &str
    {

        self.name

    }

    pub fn number_of_parameters(&self) -> usize
    {

        if let Some(number_of_parameters) = self.number_of_parameters
        {

            number_of_parameters

        }
        else
        {
            
            0

        }

    }

    pub fn is_variadic(&self) -> bool
    {

        self.number_of_parameters.is_none()

    }

}