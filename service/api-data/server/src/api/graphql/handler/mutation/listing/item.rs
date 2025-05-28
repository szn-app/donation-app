use async_graphql::{Context, Error, ErrorExtensions, Result, ResultExt};
use tracing::{debug, instrument};

use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::api::graphql::service::{DataContext, GlobalContext};
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
        ctx: &Context<'_>,
        variant: Option<ItemType>,
        intent_action: Option<ItemIntentAction>,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: Option<ItemCondition>,
        location: Option<i64>,
        created_by: Option<Uuid>,
        status: Option<ItemStatus>,
    ) -> Result<Item> {
        let c = ctx.data::<DataContext>()?;

        let user_id = c.user_id.as_ref().ok_or_else(|| {
            const ERROR_MESSAGE: &str = "Not authenticated & No user header detected";
            log::error!("{}", ERROR_MESSAGE);
            async_graphql::Error::new(ERROR_MESSAGE).extend_with(|err, e| {
                e.set("code", 401);
            })
        })?;

        let created_by =
            Some(Uuid::parse_str(user_id).expect("user_id needs to be a valid UUID string"));

        let item_repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = item_repository
            .create(
                variant,
                intent_action,
                title,
                description,
                category,
                condition,
                location,
                created_by,
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
    async fn update_item(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: Option<ItemCondition>,
        location: Option<i64>,
        status: Option<ItemStatus>,
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
