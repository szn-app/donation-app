use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{Collection, CollectionType, CollectionVisibility};
use crate::database::repository::listing::collection::CollectionRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use tracing::debug;

pub struct CollectionMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CollectionMutation {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    pub async fn create_collection(
        &self,
        _ctx: &Context<'_>,
        id_community: i64,
        title: String,
        visibility: CollectionVisibility,
        type_: CollectionType,
        position: i32,
    ) -> Result<Collection> {
        let collection_repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = collection_repository
            .create(id_community, title, visibility, type_, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(collection)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn update_collection(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: String,
        visibility: CollectionVisibility,
        type_: CollectionType,
        position: i32,
    ) -> FieldResult<Collection> {
        debug!("Updating collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .update(id, title, visibility, type_, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(collection)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_collection(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}
