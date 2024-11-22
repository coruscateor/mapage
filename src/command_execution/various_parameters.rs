use corlib::text::SendableText;

use crate::{Command, types::TypeInstance, CommandError};



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

