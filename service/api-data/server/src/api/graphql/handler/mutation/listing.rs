use crate::access_control::check_permission_for_subject;
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
    pub async fn add_category(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category> {
        let category_repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = category_repository
            .add_category(&name, &description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_category(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> FieldResult<Category> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .update_category(id, name, description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_category(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_category(id)
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
    pub async fn add_location(
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
            .add_location(
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
    async fn update_location(
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
            .update_location(id, name, address, city, state, country, postal_code)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_location(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_location(id)
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
    pub async fn add_item(
        &self,
        _ctx: &Context<'_>,
        title: String,
        description: String,
        id_category: i64,
        id_profile: Uuid,
        id_location: Option<i64>,
        price: Option<f64>,
        currency: Option<String>,
        item_type: ItemType,
        intent_action: ItemIntentAction,
        status: ItemStatus,
        condition: ItemCondition,
    ) -> Result<Item> {
        let item_repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = item_repository
            .add_item(
                &title,
                &description,
                id_category,
                id_profile,
                id_location,
                price,
                currency,
                item_type,
                intent_action,
                status,
                condition,
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
        _ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        id_category: Option<i64>,
        id_location: Option<i64>,
        price: Option<f64>,
        currency: Option<String>,
        item_type: Option<ItemType>,
        intent_action: Option<ItemIntentAction>,
        status: Option<ItemStatus>,
        condition: Option<ItemCondition>,
    ) -> FieldResult<Item> {
        debug!("Updating item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .update_item(
                id,
                title,
                description,
                id_category,
                id_location,
                price,
                currency,
                item_type,
                intent_action,
                status,
                condition,
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
    async fn delete_item(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting item: id={}", id);
        let repository = ItemRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_item(id)
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
    pub async fn add_collection(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        id_profile: Uuid,
        is_public: bool,
        collection_type: CollectionType,
    ) -> Result<Collection> {
        let collection_repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = collection_repository
            .add_collection(&name, &description, id_profile, is_public, collection_type)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_collection(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        is_public: Option<bool>,
        collection_type: Option<CollectionType>,
    ) -> FieldResult<Collection> {
        debug!("Updating collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .update_collection(id, name, description, is_public, collection_type)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_collection(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_collection(id)
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
    pub async fn add_media(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        url: String,
        media_type: MediaType,
        position: i32,
    ) -> Result<Media> {
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = media_repository
            .add_media(id_item, &url, media_type, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_media(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        url: Option<String>,
        media_type: Option<MediaType>,
        position: Option<i32>,
    ) -> FieldResult<Media> {
        debug!("Updating media: id={}", id);
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .update_media(id, url, media_type, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_media(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting media: id={}", id);
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_media(id)
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
    pub async fn add_publish(
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
            .add_publish(id_item, id_collection, note, position, created_by)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        updated_by: Uuid,
    ) -> FieldResult<Publish> {
        debug!(
            "Updating publish: item={}, collection={}",
            id_item, id_collection
        );
        let repository = PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .update_publish(id_item, id_collection, note, position, updated_by)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
    ) -> FieldResult<bool> {
        debug!(
            "Deleting publish: item={}, collection={}",
            id_item, id_collection
        );
        let repository = PublishRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_publish(id_item, id_collection)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}
