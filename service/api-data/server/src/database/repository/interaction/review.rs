use crate::database::model::interaction::Review;
use crate::database::sql::interaction::review::{
    ADD_REVIEW, GET_REVIEWS, GET_REVIEW_BY_ID, GET_REVIEW_BY_TRANSACTION_AND_SUBJECT,
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

    pub async fn get_reviews(&self) -> Result<Vec<Review>, Box<dyn Error>> {
        debug!("Getting all reviews");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_REVIEWS, &[]).await?;
        Ok(rows.into_iter().map(Review::from).collect())
    }

    pub async fn get_review_by_id(
        &self,
        id_transaction: i64,
        id_subject: i64,
    ) -> Result<Option<Review>, Box<dyn Error>> {
        debug!(
            "Getting review by transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(GET_REVIEW_BY_ID, &[&id_transaction, &id_subject])
            .await?;
        Ok(row.map(Review::from))
    }

    pub async fn get_review_by_transaction_and_subject(
        &self,
        id_transaction: i64,
        id_subject: i64,
    ) -> Result<Option<Review>, Box<dyn Error>> {
        debug!(
            "Getting review by transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_REVIEW_BY_TRANSACTION_AND_SUBJECT,
                &[&id_transaction, &id_subject],
            )
            .await?;
        Ok(row.map(Review::from))
    }

    pub async fn add_review(
        &self,
        id_transaction: i64,
        id_subject: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<Review, Box<dyn Error>> {
        debug!(
            "Adding review for transaction: {} and subject: {}",
            id_transaction, id_subject
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_REVIEW,
                &[&id_transaction, &id_subject, &rating, &comment],
            )
            .await?;
        Ok(Review::from(row))
    }
}
