use crate::{types::json::Command, CommandError, CommandResult};

pub type ExecutionResult = Result<CommandResult, CommandError>;

pub struct CommandExecutor
{



}

impl CommandExecutor
{

    pub fn new() -> Self
    {

        Self
        {
        }

    }

    async fn execute_command(&mut self, command: Command) -> ExecutionResult
    {

        todo!()

    }

}
