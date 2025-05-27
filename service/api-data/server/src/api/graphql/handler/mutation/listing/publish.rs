use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::Publish;
use crate::database::repository::listing::publish::PublishRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use uuid::Uuid;

pub struct PublishMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PublishMutation {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    pub async fn create_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        created_by: Uuid,
    ) -> Result<Publish> {
        let publish_repository = PublishRepository::new(self.postgres_pool_group.clone());
        let publish = publish_repository
            .create(id_item, id_collection, note, position, created_by)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(publish)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn update_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
    ) -> FieldResult<Publish> {
        let repository = PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .update(id_item, id_collection, note, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(publish)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
    ) -> FieldResult<bool> {
        let repository = PublishRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id_item, id_collection)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
} 