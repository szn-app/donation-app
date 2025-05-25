use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::interaction::{Message, MessageType};
use crate::database::repository::interaction::message::MessageRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, Object, Result};
use uuid::Uuid;

pub struct MessageMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MessageMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_sender: Uuid,
        type_: MessageType,
        content: String,
    ) -> Result<Message> {
        let message_repository = MessageRepository::new(self.postgres_pool_group.clone());
        let message = message_repository
            .create(id_transaction, id_sender, type_, content)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(message)
    }
} 