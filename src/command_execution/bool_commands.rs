use std::sync::Arc;

use corlib::text::SendableText;

use crate::{types::{SupportedType, TypeInstance}, Command, CommandError, CommandResult, Store};

use super::{get_key_param, take_at, ExecutionResult};

pub fn opt_bool_to_opt_ti(item: Option<bool>) -> Option<TypeInstance>
{

    if let Some(val) = item
    {

        Some(TypeInstance::Bool(val))

    }
    else
    {

        None
        
    }

}

pub async fn execute_bool_command(store: &Arc<Store>, command: Command) -> ExecutionResult
{

    match command.command.as_str()
    {

        "read" =>
        {

            let key = get_key_param(&mut command).await?;

            let res = store.bool_namespace().read(&key).await;

            match res
            {

                Ok(val) =>
                {

                    let res = CommandResult::done(&command, Some(TypeInstance::Bool(val)));
                    
                    Ok(res)

                }
                Err(err) =>
                {

                    Err(CommandError::new(&command, SendableText::String(err.to_string())))

                }

            }

        }
        "try_read" =>
        {

            let key = get_key_param(&mut command).await?;

            let result;

            if let Some(val) = store.bool_namespace().try_read(&key).await
            {

                result = Some(TypeInstance::Bool(val));

            }
            else
            {

                result = None;
                
            }

            let res = CommandResult::done(&command, result);
            
            Ok(res)


        }
        "insert" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to insert

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await //get_must_have_value_param(&command, SupportedType::Bool).await?;
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res = store.bool_namespace().insert(key, value).await;

                    match res
                    {
        
                        Ok(val) =>
                        {
        
                            let res = CommandResult::done(&command, None);

                            Ok(res)
        
                        }
                        Err(err) =>
                        {
        
                            Err(CommandError::new(&command, SendableText::String(err.to_string())))
        
                        }
        
                    }

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "update" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res = store.bool_namespace().update(&key, value).await;

                    match res
                    {
        
                        Ok(val) =>
                        {
        
                            let res = CommandResult::done(&command, None);
                            
                            Ok(res)
        
                        }
                        Err(err) =>
                        {
        
                            Err(CommandError::new(&command, SendableText::String(err.to_string())))
        
                        }
        
                    }

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "upsert" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res = store.bool_namespace().upsert(key, value).await;

                    match res
                    {
        
                        Ok(val) =>
                        {
        
                            let res = CommandResult::done(&command, None);
                            
                            Ok(res)
        
                        }
                        Err(err) =>
                        {
        
                            Err(CommandError::new(&command, SendableText::String(err.to_string())))
        
                        }
        
                    }

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "try_replace" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res_opt = store.bool_namespace().try_replace(&key, value).await;

                    let ti_opt = opt_bool_to_opt_ti(res_opt);

                    /*
                    if let Some(val) = res_opt
                    {

                        ti_opt = Some(TypeInstance::Bool(val)); 

                    }
                    else
                    {

                        ti_opt = None;
                        
                    }
                    */

                    let res = CommandResult::done(&command, ti_opt);

                    Ok(res)

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "replace" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res_opt = store.bool_namespace().replace(&key, value).await;

                    let ti_opt = opt_bool_to_opt_ti(res_opt);

                    let res = CommandResult::done(&command, ti_opt);
                    
                    Ok(res)

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "remove" =>
        {

            let key = get_key_param(&mut command).await?;

            match store.bool_namespace().remove(&key).await
            {

                Ok(_) =>
                {

                    let res = CommandResult::done(&command, None);

                    Ok(res)

                }
                Err(err) =>
                {

                    Err(CommandError::new(&command, SendableText::String(err.to_string())))

                }

            }

        }
        "try_retrieve" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res_opt = store.bool_namespace().try_retrieve(&key).await;

                    let ti_opt = opt_bool_to_opt_ti(res_opt);

                    let res = CommandResult::done(&command, ti_opt);
                    
                    Ok(res)

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "retrieve" =>
        {

            let key = get_key_param(&mut command).await?;

            //Get the value to update

            let index = 1;

            if let Some(ti_value) = take_at(&mut command, index).await
            {

                if let TypeInstance::Bool(value) = ti_value
                {

                    let res_opt = store.bool_namespace().retrieve(&key).await;

                    let ti_opt = opt_bool_to_opt_ti(res_opt);

                    let res = CommandResult::done(&command, ti_opt);
                    
                    Ok(res)

                }
                else
                {

                    Err(CommandError::at_index_with_found_type(&command, SendableText::Str("Bool Expected"), index, ti_value.get_sendable_type_name()))

                }

            }
            else
            {

                Err(CommandError::at_index(&command, SendableText::Str("Parameter not found at index"), index))
                
            }

        }
        "contains" =>
        {

            let key = get_key_param(&mut command).await?;

            let res = CommandResult::done(&command, opt_bool_to_opt_ti(Some(store.bool_namespace().contains(&key).await)));

            Ok(res)

        }
        "clear" =>
        {

            store.bool_namespace().clear().await;

            let res = CommandResult::done(&command, None);

            Ok(res)

        }
        "len" =>
        {

            let ti_res = TypeInstance::U64(store.bool_namespace().len().await as u64);

            let res = CommandResult::done(&command, Some(ti_res));

            Ok(res)

        }
        //"clear_and_get_len" =>
        "len_then_clear" =>
        {

            let ti_res = TypeInstance::U64(store.bool_namespace().len().await as u64);

            store.bool_namespace().clear().await;

            let res = CommandResult::done(&command, Some(ti_res));

            Ok(res)

        }
        "is_empty" =>
        {

            let ti_res = TypeInstance::Bool(store.bool_namespace().is_empty().await);

            let res = CommandResult::done(&command, Some(ti_res));

            Ok(res)

        }
        "capacity" =>
        {

            let ti_res = TypeInstance::U64(store.bool_namespace().capacity().await as u64);

            let res = CommandResult::done(&command, Some(ti_res));

            Ok(res)

        }
        _ =>
        {

            Err(CommandError::invalid_command_for_the_specified_type(&command))

        }
        
    }

}

