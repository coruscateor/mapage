use corlib::text::SendableText;


#[derive(Debug, Default, Clone, Copy)]
pub enum SupportedType
{

    #[default]
    Bool,
    Char,

    F32,
    F64,
    I8,
    I16,
    I32,
    I64,

    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,

    U128,
    Usize,

    //Collections

    String,
    Whatever,

    //Vecs

    VecBool,
    VecChar,

    VecF32,
    VecF64,
    VecI8,
    VecI16,
    VecI32,
    VecI64,

    VecI128,
    VecIsize,
    VecU8,
    VecU16,
    VecU32,
    VecU64,

    VecU128,
    VecUsize,

    VecString,
    VecWhatever

}

impl SupportedType
{

    pub fn try_parse(slice: &str) -> Result<SupportedType, SendableText> //&str>
    {

        match slice
        {

            "f32" => Ok(SupportedType::F32),
            "f64" => Ok(SupportedType::F64),
            "i8" => Ok(SupportedType::I8),
            "i16" => Ok(SupportedType::I16),
            "i32" => Ok(SupportedType::I32),
            "i64" => Ok(SupportedType::I64),
            "i128" => Ok(SupportedType::I128),
            "isize" => Ok(SupportedType::Isize),
            "u8" => Ok(SupportedType::U8),
            "u16" => Ok(SupportedType::U16),
            "u32" => Ok(SupportedType::U32),
            "u64" => Ok(SupportedType::U64),
            "u128" => Ok(SupportedType::U128),
            "usize" => Ok(SupportedType::Usize),

    //Collections

            "string" => Ok(SupportedType::String),
            "whatever" => Ok(SupportedType::Whatever),

    //Vecs

            "vec_bool" => Ok(SupportedType::VecBool),
            "vec_char" => Ok(SupportedType::VecChar),
            "vec_f32" => Ok(SupportedType::VecF32),
            "vec_f64" => Ok(SupportedType::VecF64),
            "vec_i8" => Ok(SupportedType::VecI8),
            "vec_i16" => Ok(SupportedType::I16),
            "vec_i32" => Ok(SupportedType::VecI32),
            "vec_i64" => Ok(SupportedType::VecI64),
            "vec_i128" => Ok(SupportedType::VecI128),
            "vec_isize" => Ok(SupportedType::VecIsize),
            "vec_u8" => Ok(SupportedType::U8),
            "vec_u16" => Ok(SupportedType::U16),
            "vec_u32" => Ok(SupportedType::U32),
            "vec_u64" => Ok(SupportedType::U64),
            "vec_u128" => Ok(SupportedType::U128),
            "vec_usize" => Ok(SupportedType::VecUsize),
            "vec_string" => Ok(SupportedType::VecString),
            "vec_whatever" => Ok(SupportedType::VecWhatever),
            //_ => Err("Provided type not recognised.")
            _ => Err(SendableText::Str("Provided value is not a suipported type."))

        }

    }

}

