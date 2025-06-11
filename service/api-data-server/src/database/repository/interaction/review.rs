use crate::database::model::interaction::Review;
use crate::database::sql::interaction::review::{
    CREATE_REVIEW, LIST_REVIEWS, FIND_REVIEW, FIND_REVIEW_BY_TRANSACTION_AND_SUBJECT,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct ReviewRepository {
    pool: PostgresPool,
}

impl ReviewRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Review>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all reviews");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_REVIEWS, &[]).await?;
        Ok(rows.into_iter().map(Review::from).collect())
    }

    pub async fn find(
        &self,
        id_transaction: i64,
        id_subject: i64,
    ) -> Result<Option<Review>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Getting review by transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(FIND_REVIEW, &[&id_transaction, &id_subject])
            .await?;
        Ok(row.map(Review::from))
    }

    pub async fn find_by_transaction_and_subject(
        &self,
        id_transaction: i64,
        id_subject: i64,
    ) -> Result<Option<Review>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Getting review by transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                FIND_REVIEW_BY_TRANSACTION_AND_SUBJECT,
                &[&id_transaction, &id_subject],
            )
            .await?;
        Ok(row.map(Review::from))
    }

    pub async fn create(
        &self,
        id_transaction: i64,
        id_subject: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<Review, Box<dyn Error + Send + Sync>> {
        debug!(
            "Adding review for transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_REVIEW,
                &[&id_transaction, &id_subject, &rating, &comment],
            )
            .await?;
        Ok(Review::from(row))
    }
}
