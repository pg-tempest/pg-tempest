use async_trait::async_trait;

pub trait Command {
    type Response;
}

#[async_trait]
pub trait CommandHandler<TCommand: Command>: Send + Sync {
    async fn handle(&self, command: TCommand) -> TCommand::Response;
}