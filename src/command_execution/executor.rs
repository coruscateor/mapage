use std::sync::Arc;

use corlib::text::SendableText;

use crate::{Command, types::{SupportedType, TypeInstance}, CommandError, CommandResult, Store};

use super::execute_bool_command;

pub type ExecutionResult = Result<CommandResult, CommandError>;

pub struct CommandExecutor
{

    store: Arc<Store>

}

impl CommandExecutor
{

    pub fn new(store: Arc<Store>) -> Self
    {

        Self
        {

            store

        }

    }

    /*
    async fn execute_command(&mut self, command: Command) -> ExecutionResult
    {

        todo!()

    }
    */

    pub async fn execute_command(&mut self, command: Command) -> ExecutionResult //Result<(), SendableText>
    {

        if let Some(type_name) = command.type_name
        {

            match type_name
            {

                SupportedType::Bool => execute_bool_command(&self.store, command).await,
                SupportedType::Char => todo!(),
                SupportedType::F32 => todo!(),
                SupportedType::F64 => todo!(),
                SupportedType::I8 => todo!(),
                SupportedType::I16 => todo!(),
                SupportedType::I32 => todo!(),
                SupportedType::I64 => todo!(),
                SupportedType::I128 => todo!(),
                //SupportedType::Isize => todo!(),
                SupportedType::U8 => todo!(),
                SupportedType::U16 => todo!(),
                SupportedType::U32 => todo!(),
                SupportedType::U64 => todo!(),
                SupportedType::U128 => todo!(),
                //SupportedType::Usize => todo!(),
                SupportedType::String => todo!(),
                SupportedType::Whatever => todo!(),
                SupportedType::VecBool => todo!(),
                //SupportedType::VecChar => todo!(),
                SupportedType::VecF32 => todo!(),
                SupportedType::VecF64 => todo!(),
                SupportedType::VecI8 => todo!(),
                SupportedType::VecI16 => todo!(),
                SupportedType::VecI32 => todo!(),
                SupportedType::VecI64 => todo!(),
                SupportedType::VecI128 => todo!(),
                //SupportedType::VecIsize => todo!(),
                SupportedType::VecU8 => todo!(),
                SupportedType::VecU16 => todo!(),
                SupportedType::VecU32 => todo!(),
                SupportedType::VecU64 => todo!(),
                SupportedType::VecU128 => todo!(),
                //SupportedType::VecUsize => todo!(),
                //SupportedType::VecString => todo!(),
                //SupportedType::VecWhatever => todo!(),
                _ =>
                {

                    Err(CommandError::invalid_command(&command))

                }

            }

            //Ok(())
            
        }
        else
        {

            match command.command.as_str()
            {

                "features" =>
                {

                    todo!()

                }
                "ser_for" =>
                {

                    todo!()

                }
                _ =>
                {

                    Err(CommandError::invalid_command(&command))

                }
                
            }

            //Err(CommandError::command_must_have_type(&command))

        }

    }

}
