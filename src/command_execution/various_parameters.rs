//use std::ops::Index;

use corlib::text::SendableText;

use crate::{types::{SupportedType, TypeInstance}, Command, CommandError};

use paste::paste;

pub async fn get_key_param(command: &Command) -> Result<&String, CommandError> //SendableText>
{

    if let Some(params) = &command.params
    {

        if let Some(opt_key) = params.first()
        {

            if let Some(ti_key) = opt_key
            {

                match ti_key
                {

                    TypeInstance::String(key) =>
                    {

                        Ok(key)

                    }
                    _ =>
                    {

                        Err(CommandError::new(command.id, SendableText::Str("The provided key parameter is the wrong type.")))

                        //Err(SendableText::Str("The provided key parameter is the wrong type."))

                    }
                    
                }

            }
            else
            {

                Err(CommandError::new(command.id, SendableText::Str("Key not provided.")))

                //Err(SendableText::Str("Key not provided."))

            }

        }
        else
        {

            //Error: parameter list empty.
            
            Err(CommandError::new(command.id, SendableText::Str("Provided parameter list empty.")))

            //Err(SendableText::Str("Provided parameter list empty."))

        }

    }
    else
    {

        Err(CommandError::new(command.id, SendableText::Str("No parameter list list provided.")))

        //Err(SendableText::Str("No parameter list list provided."))
        
    }

}

macro_rules! impl_is_type_method
{

    ($ti_value:ident, $command:ident, $type_fn_name:ty) =>
    {

        paste!
        {

            if $ti_value.[<is_ $type_fn_name>]()
            {

                Ok($ti_value)
                
            }
            else
            {

                wrong_type_error($command)
                
            }

        }

    }

}

pub async fn get_must_have_value_param(command: &Command, of_type: SupportedType) -> Result<&TypeInstance, CommandError>
{

    if let Some(params) = &command.params
    {

        if let Some(opt_value) = params.get(1)
        {

            if let Some(ti_value) = opt_value
            {

                match of_type
                {

                    SupportedType::Bool =>
                    {

                        impl_is_type_method!(ti_value, command, bool)

                        /*
                        if ti_value.is_bool()
                        {

                            Ok(ti_value)
                            
                        }
                        else
                        {

                            wrong_type_error(command)
                            
                        }
                        */

                    }
                    SupportedType::Char =>
                    {

                        impl_is_type_method!(ti_value, command, char)

                    }
                    SupportedType::F32 =>
                    {

                        impl_is_type_method!(ti_value, command, f32)

                    }
                    SupportedType::F64 =>
                    {

                        impl_is_type_method!(ti_value, command, f64)

                    }
                    SupportedType::I8 =>
                    {

                        impl_is_type_method!(ti_value, command, i8)

                    }
                    SupportedType::I16 =>
                    {

                        impl_is_type_method!(ti_value, command, i16)

                    }
                    SupportedType::I32 =>
                    {

                        impl_is_type_method!(ti_value, command, i32)

                    }
                    SupportedType::I64 =>
                    {

                        impl_is_type_method!(ti_value, command, i64)

                    }
                    SupportedType::I128 =>
                    {

                        impl_is_type_method!(ti_value, command, i128)

                    }
                    //SupportedType::Isize => todo!(),
                    SupportedType::U8 =>
                    {

                        impl_is_type_method!(ti_value, command, u8)

                    }
                    SupportedType::U16 =>
                    {

                        impl_is_type_method!(ti_value, command, i16)

                    }
                    SupportedType::U32 =>
                    {

                        impl_is_type_method!(ti_value, command, i32)

                    }
                    SupportedType::U64 =>
                    {

                        impl_is_type_method!(ti_value, command, i64)

                    }
                    SupportedType::U128 =>
                    {

                        impl_is_type_method!(ti_value, command, i128)

                    }
                    //SupportedType::Usize => todo!(),
                    SupportedType::String =>
                    {

                        impl_is_type_method!(ti_value, command, string)

                    }
                    SupportedType::Whatever =>
                    {

                        //impl_is_type_method!(ti_value, command, whatever)

                        Ok(ti_value)

                    }
                    SupportedType::VecBool =>
                    {

                        impl_is_type_method!(ti_value, command, vec_bool)

                    }
                    //SupportedType::VecChar => todo!(),
                    SupportedType::VecF32 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_f32)

                    }
                    SupportedType::VecF64 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_f64)

                    }
                    SupportedType::VecI8 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_i8)

                    }
                    SupportedType::VecI16 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_i16)

                    }
                    SupportedType::VecI32 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_i32)

                    }
                    SupportedType::VecI64 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_i64)

                    }
                    SupportedType::VecI128 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_i128)

                    }
                    //SupportedType::VecIsize => todo!(),
                    SupportedType::VecU8 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_u8)

                    }
                    SupportedType::VecU16 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_u16)

                    }
                    SupportedType::VecU32 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_u32)

                    }
                    SupportedType::VecU64 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_u64)

                    }
                    SupportedType::VecU128 =>
                    {

                        impl_is_type_method!(ti_value, command, vec_u128)

                    }
                    //SupportedType::VecUsize => todo!(),
                    //SupportedType::VecString => todo!(),
                    //SupportedType::VecWhatever => todo!(),
                }

                
                

            }
            else
            {

                Err(CommandError::new(command.id, SendableText::Str("Key not provided.")))

            }

        }
        else
        {

            //Error: parameter list empty.
            
            Err(CommandError::new(command.id, SendableText::Str("Provided parameter list empty.")))

        }

    }
    else
    {

        Err(CommandError::new(command.id, SendableText::Str("No parameter list list provided.")))
        
    }

}

fn wrong_type_error<T>(command: &Command) -> Result<T, CommandError>
{

    Err(CommandError::new(command.id, SendableText::Str("The provided value parameter is the wrong type.")))

}
