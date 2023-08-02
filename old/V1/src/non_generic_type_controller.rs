use crate::memrs_type::*;

pub struct ScalarTypeController
{

    bool_type: RcdType,
    char_type: RcdType,
    f32_type: RcdType,
    f64_type: RcdType,
    i8_type: RcdType,
    i16_type: RcdType,
    i32_type: RcdType,
    i64_type: RcdType,
    i128_type: RcdType,
    isize_type: RcdType,
    u8_type: RcdType,
    u16_type: RcdType,
    u32_type: RcdType,
    u64_type: RcdType,
    u128_type: RcdType,
    unit_type: RcdType,
    usize_type: RcdType,
    nothing_type: RcdType, //non-standard
    //optional_type: RcdType,
    //required_type: RcdType,
    //something_type: RcdType,
    whatever_type: RcdType
}

impl ScalarTypeController
{

    pub fn new() -> Self
    {

        Self
        {

            bool_type: Rcd::new(Type::new_bool()),
            char_type: Rcd::new(Type::new_char()),
            f32_type: Rcd::new(Type::new_f32()),
            f64_type: Rcd::new(Type::new_f64()),
            i8_type: Rcd::new(Type::new_i8()),
            i16_type: Rcd::new(Type::new_i16()),
            i32_type: Rcd::new(Type::new_i32()),
            i64_type: Rcd::new(Type::new_i64()),
            i128_type: Rcd::new(Type::new_i128()),
            isize_type: Rcd::new(Type::new_isize()),
            u8_type: Rcd::new(Type::new_u8()),
            u16_type: Rcd::new(Type::new_u16()),
            u32_type: Rcd::new(Type::new_u32()),
            u64_type: Rcd::new(Type::new_u64()),
            u128_type: Rcd::new(Type::new_u128()),
            unit_type: Rcd::new(Type::new_unit()),
            usize_type: Rcd::new(Type::new_usize()),
            nothing_type: Rcd::new(Type::new_nothing()),
            //optional_type: Rcd::new(Type::new_optional()),
            //required_type: Rcd::new(Type::new_required()),
            //something_type: Rcd::new(Type::new_something()),
            whatever_type: Rcd::new(Type::new_whatever())

        }
        
    }

    //Types

    pub fn get_bool_type(&self) -> RcdType
    {

        self.bool_type.clone()

    }

    pub fn get_char_type(&self) -> RcdType
    {

        self.char_type.clone()

    }

    pub fn get_f32_type(&self) -> RcdType
    {

        self.f32_type.clone()

    }

    pub fn get_f64_type(&self) -> RcdType
    {

        self.f64_type.clone()

    }

    pub fn get_i8_type(&self) -> RcdType
    {

        self.i8_type.clone()

    }

    pub fn get_i16_type(&self) -> RcdType
    {

        self.i16_type.clone()

    }

    pub fn get_i32_type(&self) -> RcdType
    {

        self.i32_type.clone()

    }

    pub fn get_i64_type(&self) -> RcdType
    {

        self.i64_type.clone()

    }

    pub fn get_i128_type(&self) -> RcdType
    {

        self.i128_type.clone()

    }

    pub fn get_isize_type(&self) -> RcdType
    {

        self.isize_type.clone()

    }

    pub fn get_u8_type(&self) -> RcdType
    {

        self.u8_type.clone()

    }

    pub fn get_u16_type(&self) -> RcdType
    {

        self.u16_type.clone()

    }

    pub fn get_u32_type(&self) -> RcdType
    {

        self.u32_type.clone()

    }

    pub fn get_u64_type(&self) -> RcdType
    {

        self.u64_type.clone()

    }

    pub fn get_u128_type(&self) -> RcdType
    {

        self.u128_type.clone()

    }

    pub fn get_unit_type(&self) -> RcdType
    {

        self.unit_type.clone()

    }

    pub fn get_usize_type(&self) -> RcdType
    {

        self.usize_type.clone()

    }

    //non-standard

    pub fn get_nothing_type(&self) -> RcdType
    {

        self.nothing_type.clone()

    }
    
    /*
    pub fn get_optional_type(&self) -> RcdType
    {

        self.optional_type.clone()

    }

    pub fn get_required_type(&self) -> RcdType
    {

        self.required_type.clone()

    }

    pub fn get_something_type(&self) -> RcdType
    {

        self.something_type.clone()

    }
    */
    
    pub fn get_whatever_type(&self) -> RcdType
    {

        self.whatever_type.clone()

    }

    

}

pub static SCALAR_TYPE_CONTROLLER: ScalarTypeController = ScalarTypeController::new();

pub fn get_STC() -> &'static ScalarTypeController
{

    &SCALAR_TYPE_CONTROLLER

}
