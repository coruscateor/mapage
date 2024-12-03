//use std::ops::Index;

use corlib::text::SendableText;

use crate::{types::{SupportedType, TypeInstance, TypeInstanceConversionError}, Command, CommandError};

use paste::paste;

//Get vale at index 0.

pub async fn get_key_param(command: &mut Command) -> Result<String, CommandError> //SendableText>
{

    if let Some(params) = &mut command.params
    {

        if let Some(opt_key) = params.first_mut()
        {

            if let Some(ti_key) = opt_key.take()
            {

                match ti_key
                {

                    TypeInstance::String(key) =>
                    {

                        Ok(key)

                    }
                    _ =>
                    {

                        Err(CommandError::new(command, SendableText::Str("The provided key parameter is the wrong type.")))

                        //Err(SendableText::Str("The provided key parameter is the wrong type."))

                    }
                    
                }

            }
            else
            {

                Err(CommandError::new(command, SendableText::Str("Key not provided.")))

                //Err(SendableText::Str("Key not provided."))

            }

        }
        else
        {

            //Error: parameter list empty.
            
            Err(CommandError::new(command, SendableText::Str("Provided parameter list empty.")))

            //Err(SendableText::Str("Provided parameter list empty."))

        }

    }
    else
    {

        Err(CommandError::new(command, SendableText::Str("No parameter list list provided.")))

        //Err(SendableText::Str("No parameter list list provided."))
        
    }

}

/*
macro_rules! is_type_or_error
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
*/

pub async fn take_at(command: &mut Command, index: usize) -> Option<TypeInstance>
{

    if let Some(params) = &mut command.params
    {

        if let Some(opt_value) = params.get_mut(index)
        {

            return opt_value.take();

        }

    }

    None

}



/*
pub async fn get_must_have_value_param<T>(command: &mut Command, of_type: SupportedType) -> Result<T, CommandError> //Result<&TypeInstance, CommandError>
    where T: From<bool>
{

    if let Some(params) = &command.params
    {

        if let Some(opt_value) = params.get_mut(1)
        {

            if let Some(ti_value) = opt_value.take()
            {

                match of_type
                {

                    SupportedType::Bool =>
                    {

                        let try_res = ti_value.take_bool(); //: Result<T, TypeInstanceConversionError> //.try_into();

                        match try_res
                        {
                            Ok(res) => Ok(res.into()),
                            Err(_) => todo!(),
                        }

                        //Ok(res)

                        //is_type_or_error!(ti_value, command, bool)

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

                        is_type_or_error!(ti_value, command, char)

                    }
                    SupportedType::F32 =>
                    {

                        is_type_or_error!(ti_value, command, f32)

                    }
                    SupportedType::F64 =>
                    {

                        is_type_or_error!(ti_value, command, f64)

                    }
                    SupportedType::I8 =>
                    {

                        is_type_or_error!(ti_value, command, i8)

                    }
                    SupportedType::I16 =>
                    {

                        is_type_or_error!(ti_value, command, i16)

                    }
                    SupportedType::I32 =>
                    {

                        is_type_or_error!(ti_value, command, i32)

                    }
                    SupportedType::I64 =>
                    {

                        is_type_or_error!(ti_value, command, i64)

                    }
                    SupportedType::I128 =>
                    {

                        is_type_or_error!(ti_value, command, i128)

                    }
                    //SupportedType::Isize => todo!(),
                    SupportedType::U8 =>
                    {

                        is_type_or_error!(ti_value, command, u8)

                    }
                    SupportedType::U16 =>
                    {

                        is_type_or_error!(ti_value, command, i16)

                    }
                    SupportedType::U32 =>
                    {

                        is_type_or_error!(ti_value, command, i32)

                    }
                    SupportedType::U64 =>
                    {

                        is_type_or_error!(ti_value, command, i64)

                    }
                    SupportedType::U128 =>
                    {

                        is_type_or_error!(ti_value, command, i128)

                    }
                    //SupportedType::Usize => todo!(),
                    SupportedType::String =>
                    {

                        is_type_or_error!(ti_value, command, string)

                    }
                    SupportedType::Whatever =>
                    {

                        //is_type_or_error!(ti_value, command, whatever)

                        Ok(ti_value)

                    }
                    SupportedType::VecBool =>
                    {

                        is_type_or_error!(ti_value, command, vec_bool)

                    }
                    //SupportedType::VecChar => todo!(),
                    SupportedType::VecF32 =>
                    {

                        is_type_or_error!(ti_value, command, vec_f32)

                    }
                    SupportedType::VecF64 =>
                    {

                        is_type_or_error!(ti_value, command, vec_f64)

                    }
                    SupportedType::VecI8 =>
                    {

                        is_type_or_error!(ti_value, command, vec_i8)

                    }
                    SupportedType::VecI16 =>
                    {

                        is_type_or_error!(ti_value, command, vec_i16)

                    }
                    SupportedType::VecI32 =>
                    {

                        is_type_or_error!(ti_value, command, vec_i32)

                    }
                    SupportedType::VecI64 =>
                    {

                        is_type_or_error!(ti_value, command, vec_i64)

                    }
                    SupportedType::VecI128 =>
                    {

                        is_type_or_error!(ti_value, command, vec_i128)

                    }
                    //SupportedType::VecIsize => todo!(),
                    SupportedType::VecU8 =>
                    {

                        is_type_or_error!(ti_value, command, vec_u8)

                    }
                    SupportedType::VecU16 =>
                    {

                        is_type_or_error!(ti_value, command, vec_u16)

                    }
                    SupportedType::VecU32 =>
                    {

                        is_type_or_error!(ti_value, command, vec_u32)

                    }
                    SupportedType::VecU64 =>
                    {

                        is_type_or_error!(ti_value, command, vec_u64)

                    }
                    SupportedType::VecU128 =>
                    {

                        is_type_or_error!(ti_value, command, vec_u128)

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
*/

/*
pub async fn get_must_have_value_param<bool>(command: &mut Command, of_type: SupportedType) -> Result<bool, CommandError>
{

    Ok(true)

}
*/

fn wrong_type_error<T>(command: &Command) -> Result<T, CommandError>
{

    Err(CommandError::new(command, SendableText::Str("The provided value parameter is the wrong type.")))

}
