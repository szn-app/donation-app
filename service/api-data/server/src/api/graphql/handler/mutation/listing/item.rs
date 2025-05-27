use async_graphql::{Context, Error, Result};
use tracing::{debug, instrument};

use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{
    Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType,
};
use crate::database::repository::listing::item::ItemRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, FieldResult};
use log;
use uuid::Uuid;

pub struct ItemMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ItemMutation {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    pub async fn create_item(
        &self,
        _ctx: &Context<'_>,
        type_: ItemType,
        intent_action: ItemIntentAction,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: ItemCondition,
        location: Option<i64>,
        created_by: Option<Uuid>,
    ) -> Result<Item> {
        let item_repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = item_repository
            .create(
                type_,
                intent_action,
                title,
                description,
                category,
                condition,
                location,
                created_by,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(item)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn update_item(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: ItemCondition,
        location: Option<i64>,
        status: ItemStatus,
    ) -> FieldResult<Item> {
        debug!("Updating item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .update(
                id,
                title,
                description,
                category,
                condition,
                location,
                status,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(item)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_item(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        let result = repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(result)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn report_item(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<Item>> {
        debug!("Reporting item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .report(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(item)
    }
}
