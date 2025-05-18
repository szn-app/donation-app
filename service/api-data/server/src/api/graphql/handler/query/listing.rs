use crate::api::graphql::access_control::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};

use async_graphql::{self, Context, Error, ErrorExtensions, FieldResult, Object};
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;
use time;
use uuid;

pub struct CategoryQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CategoryQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_categories(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Category>> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let categories = repository
            .get_categories()
            .await
            .map_err(|e| e.to_string())?;
        Ok(categories)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::Category>> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .get_category_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_categories_by_parent(
        &self,
        ctx: &Context<'_>,
        parent_id: Option<i64>,
    ) -> FieldResult<Vec<model::Category>> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let categories = repository
            .get_categories_by_parent(parent_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(categories)
    }
}

pub struct LocationQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl LocationQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_locations(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Location>> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let locations = repository
            .get_locations()
            .await
            .map_err(|e| e.to_string())?;
        Ok(locations)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_location_by_id(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::Location>> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository
            .get_location_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_locations_by_profile(
        &self,
        ctx: &Context<'_>,
        profile_id: uuid::Uuid,
    ) -> FieldResult<Vec<model::Location>> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let locations = repository
            .get_locations_by_profile(profile_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(locations)
    }
}

pub struct ItemQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ItemQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_items(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let items = repository.get_items().await.map_err(|e| e.to_string())?;
        Ok(items)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_item_by_id(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .get_item_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(item)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_items_by_profile(
        &self,
        ctx: &Context<'_>,
        profile_id: uuid::Uuid,
    ) -> FieldResult<Vec<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let items = repository
            .get_items_by_profile(profile_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(items)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_items_by_category(
        &self,
        ctx: &Context<'_>,
        category_id: i64,
    ) -> FieldResult<Vec<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let items = repository
            .get_items_by_category(category_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(items)
    }
}

pub struct CollectionQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CollectionQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_collections(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Collection>> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collections = repository
            .get_collections()
            .await
            .map_err(|e| e.to_string())?;
        Ok(collections)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_collection_by_id(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::Collection>> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .get_collection_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_collections_by_profile(
        &self,
        ctx: &Context<'_>,
        profile_id: uuid::Uuid,
    ) -> FieldResult<Vec<model::Collection>> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collections = repository
            .get_collections_by_profile(profile_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(collections)
    }
}

pub struct MediaQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MediaQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_media(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Media>> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository.get_media().await.map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_media_by_id(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::Media>> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .get_media_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_media_by_item(
        &self,
        ctx: &Context<'_>,
        item_id: i64,
    ) -> FieldResult<Vec<model::Media>> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .get_media_by_item(item_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(media)
    }
}

pub struct PublishQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PublishQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_publishes(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Publish>> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publishes = repository
            .get_publishes()
            .await
            .map_err(|e| e.to_string())?;
        Ok(publishes)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_publish_by_id(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::Publish>> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .get_publish_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_publish_by_item_and_collection(
        &self,
        ctx: &Context<'_>,
        item_id: i64,
        collection_id: i64,
    ) -> FieldResult<Option<model::Publish>> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .get_publish_by_item_and_collection(item_id, collection_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_publishes_by_collection(
        &self,
        ctx: &Context<'_>,
        collection_id: i64,
    ) -> FieldResult<Vec<model::Publish>> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publishes = repository
            .get_publishes_by_collection(collection_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publishes)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_publishes_by_item(
        &self,
        ctx: &Context<'_>,
        item_id: i64,
    ) -> FieldResult<Vec<model::Publish>> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publishes = repository
            .get_publishes_by_item(item_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publishes)
    }
}
