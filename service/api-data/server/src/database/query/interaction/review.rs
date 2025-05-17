use crate::database::model::interaction::Review;
use crate::database::sql::{GET_REVIEWS, GET_REVIEW_BY_TRANSACTION_AND_SUBJECT};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;

pub struct ReviewRepository {
    pool: Pool,
}

impl ReviewRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_reviews(&self) -> Result<Vec<Review>> {
        debug!("Getting all reviews");
        let client = self.pool.get().await?;
        let rows = client.query(GET_REVIEWS, &[]).await?;
        Ok(rows.into_iter().map(Review::from).collect())
    }

    pub async fn get_review_by_transaction_and_subject(
        &self,
        id_transaction: i64,
        id_subject_profile: i64,
    ) -> Result<Option<Review>> {
        debug!(
            "Getting review by transaction: {} and subject: {}",
            id_transaction, id_subject_profile
        );
        let client = self.pool.get().await?;
        let row = client
            .query_opt(
                GET_REVIEW_BY_TRANSACTION_AND_SUBJECT,
                &[&id_transaction, &id_subject_profile],
            )
            .await?;
        Ok(row.map(Review::from))
    }
}
