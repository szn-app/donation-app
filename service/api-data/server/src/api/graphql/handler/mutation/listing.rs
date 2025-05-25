use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{
    Category, Collection, CollectionType, CollectionVisibility, Item, ItemCondition,
    ItemIntentAction, ItemStatus, ItemType, Location, Media, MediaType, Publish,
};
use crate::database::repository::listing::{
    category::CategoryRepository, collection::CollectionRepository, item::ItemRepository,
    location::LocationRepository, media::MediaRepository, publish::PublishRepository,
};
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, Error, FieldResult, Object, Result};
use log;
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

pub struct CategoryMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CategoryMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category> {
        let category_repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = category_repository
            .create(&name, &description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> FieldResult<Category> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .update(id, name, description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct LocationMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl LocationMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        name: String,
        address: String,
        city: String,
        state: String,
        country: String,
        postal_code: String,
        id_profile: Uuid,
    ) -> Result<Location> {
        let location_repository = LocationRepository::new(self.postgres_pool_group.clone());
        let location = location_repository
            .create(
                &name,
                &address,
                &city,
                &state,
                &country,
                &postal_code,
                id_profile,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
    ) -> FieldResult<Location> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository
            .update(id, name, address, city, state, country, postal_code)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}

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
    pub async fn create(
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
    async fn update(
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
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("Item not found"))?;
        Ok(item)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct CollectionMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CollectionMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
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
            .create(
                id_community,
                title.to_string(),
                visibility,
                type_,
                position,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: String,
        visibility: CollectionVisibility,
        type_: CollectionType,
        position: i32,
    ) -> FieldResult<Collection> {
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .update(id, title, visibility, type_, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct MediaMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MediaMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        url: String,
        media_type: MediaType,
        position: i32,
    ) -> Result<Media> {
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = media_repository
            .create(
                id_item,
                None,
                url.to_string(),
                media_type,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        url: Option<String>,
        media_type: Option<MediaType>,
        position: Option<i32>,
    ) -> FieldResult<Media> {
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .update(
                id,
                None,
                url.unwrap_or_default(),
                media_type.unwrap_or_default()
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct PublishMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PublishMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
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

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
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

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(
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
