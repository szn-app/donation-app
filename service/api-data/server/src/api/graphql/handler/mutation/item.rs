use async_graphql::{Context, Error, Result};
use tracing::{debug, instrument};

use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{
    Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType,
};
use crate::database::repository::item::ItemRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, FieldResult};
use log;
use uuid::Uuid;

pub struct ItemMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ItemMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
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

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_item(
        &self,
        ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        condition: Option<String>,
        quantity: Option<i32>,
    ) -> FieldResult<Item> {
        debug!("Updating item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .update_item(id, title, description, condition, quantity)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(item)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_item(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_item(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}
