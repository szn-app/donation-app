use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::interaction::Review;
use crate::database::repository::interaction::review::ReviewRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, Object, Result};

pub struct ReviewMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ReviewMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_subject: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<Review> {
        let review_repository = ReviewRepository::new(self.postgres_pool_group.clone());
        let review = review_repository
            .create(id_transaction, id_subject, rating, comment)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(review)
    }
} 