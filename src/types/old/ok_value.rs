use std::mem::size_of_val;

const OK: &str = "Ok";

pub fn get_ok_value_str() -> &'static str
{

    OK

}

/*
#[derive(SimpleObject, Clone, Copy, Default)]
pub struct OkValue
{

    value: &'static str

}

impl OkValue
{

    pub fn new() -> Self
    {

        Self
        {

            value: OK

        }

    }

    pub fn get_value(&self) -> &str
    {

        self.value

    }

}

pub fn size_of_ok_value_str() -> usize
{

    size_of_val(OK)

}
*/
