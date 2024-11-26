use std::sync::Arc;

use corlib::text::SendableText;

use crate::{types::{SupportedType, TypeInstance}, Command, CommandError, CommandResult, Store};

use super::{get_key_param, get_must_have_value_param, ExecutionResult};



pub async fn execute_bool_command(store: &Arc<Store>, command: Command) -> ExecutionResult
{

    match command.command.as_str()
    {

        "read" =>
        {

            let key = get_key_param(&command).await?;

            let res = store.bool_namespace().read(key).await;

            match res
            {

                Ok(val) =>
                {

                    let res = CommandResult
                    {

                        id: command.id,
                        result: Some(TypeInstance::Bool(val)),
                        message: None,
                        done: true

                    };
                    
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

            let key = get_key_param(&command).await?;

            let result;

            if let Some(val) = store.bool_namespace().try_read(key).await
            {

                result = Some(TypeInstance::Bool(val));

            }
            else
            {

                result = None;
                
            }

            let res = CommandResult
            {

                id: command.id,
                result,
                message: None,
                done: true

            };
            
            Ok(res)


        }
        "insert" =>
        {

            let key = get_key_param(&command).await?;

            let value = get_must_have_value_param(&command, SupportedType::Bool).await?;

            let res = store.bool_namespace().insert(key).await;

            match res
            {

                Ok(val) =>
                {

                    let res = CommandResult
                    {

                        id: command.id,
                        result: Some(TypeInstance::Bool(val)),
                        message: None,
                        done: true

                    };
                    
                    Ok(res)

                }
                Err(err) =>
                {

                    Err(CommandError::new(command.id, SendableText::String(err.to_string())))

                }

            }

        }
        "update" =>
        {



        }
        "upsert" =>
        {

            

        }
        "try_replace" =>
        {



        }
        "replace" =>
        {



        }
        "remove" =>
        {



        }
        "try_retrieve" =>
        {



        }
        "retrieve" =>
        {



        }
        "contains" =>
        {



        }
        "clear" =>
        {



        }
        "len" =>
        {



        }
        //"clear_and_get_len" =>
        "len_then_clear" =>
        {



        }
        "is_empty" =>
        {



        }
        "capacity" =>
        {

            Err(CommandError::not_implemented(&command))

        }
        _ =>
        {

            Err(CommandError::invalid_command_for_the_specified_type(&command))

        }
        
    }

}

