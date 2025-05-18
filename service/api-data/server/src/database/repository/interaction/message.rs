use crate::database::model::interaction::{Message, MessageType};
use crate::database::sql::{
    ADD_MESSAGE, GET_MESSAGES, GET_MESSAGES_BY_TRANSACTION, GET_MESSAGE_BY_ID,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct MessageRepository {
    pool: PostgresPool,
}

impl MessageRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_messages(&self) -> Result<Vec<Message>, Box<dyn Error>> {
        debug!("Getting all messages");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_MESSAGES, &[]).await?;
        Ok(rows.into_iter().map(Message::from).collect())
    }

    pub async fn get_message_by_id(&self, id: i64) -> Result<Option<Message>, Box<dyn Error>> {
        debug!("Getting message by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_MESSAGE_BY_ID, &[&id]).await?;
        Ok(row.map(Message::from))
    }

    pub async fn get_messages_by_transaction(
        &self,
        id_transaction: i64,
    ) -> Result<Vec<Message>, Box<dyn Error>> {
        debug!("Getting messages by transaction: {}", id_transaction);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_MESSAGES_BY_TRANSACTION, &[&id_transaction])
            .await?;
        Ok(rows.into_iter().map(Message::from).collect())
    }

    pub async fn add_message(
        &self,
        id_transaction: i64,
        id_sender: Uuid,
        type_: MessageType,
        content: String,
    ) -> Result<Message, Box<dyn Error>> {
        debug!("Adding message to transaction: {}", id_transaction);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_MESSAGE,
                &[&id_transaction, &id_sender, &type_, &content],
            )
            .await?;
        Ok(Message::from(row))
    }
}
