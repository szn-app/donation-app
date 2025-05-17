use crate::database::model::interaction::{Message, MessageType};
use crate::database::sql::{
    ADD_MESSAGE, GET_MESSAGES, GET_MESSAGES_BY_TRANSACTION, GET_MESSAGE_BY_ID,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;

pub struct MessageRepository {
    pool: Pool,
}

impl MessageRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_messages(&self) -> Result<Vec<Message>> {
        debug!("Getting all messages");
        let client = self.pool.get().await?;
        let rows = client.query(GET_MESSAGES, &[]).await?;
        Ok(rows.into_iter().map(Message::from).collect())
    }

    pub async fn get_message_by_id(&self, id: i64) -> Result<Option<Message>> {
        debug!("Getting message by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_MESSAGE_BY_ID, &[&id]).await?;
        Ok(row.map(Message::from))
    }

    pub async fn get_messages_by_transaction(&self, id_transaction: i64) -> Result<Vec<Message>> {
        debug!("Getting messages by transaction: {}", id_transaction);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_MESSAGES_BY_TRANSACTION, &[&id_transaction])
            .await?;
        Ok(rows.into_iter().map(Message::from).collect())
    }

    pub async fn add_message(
        &self,
        id_sender: i64,
        id_transaction: i64,
        type_: MessageType,
        content: &str,
    ) -> Result<Message> {
        debug!("Adding message for transaction: {}", id_transaction);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_MESSAGE,
                &[&id_sender, &id_transaction, &type_, &content],
            )
            .await?;
        Ok(Message::from(row))
    }
}
