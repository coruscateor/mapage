use std::sync::Arc;

use corlib::text::SendableText;

use crate::{Command, types::TypeInstance, CommandError, CommandResult, Store};

use super::{get_key_param, ExecutionResult};



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
                        result: TypeInstance::Bool(val),
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
        "try_read" =>
        {



        }
        "insert" =>
        {



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

