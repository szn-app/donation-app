use crate::api::graphql::guard::{auth, AuthorizeUser};
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
    async fn list_categories(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Category>> {
        let repository = repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let categories = repository.list().await.map_err(|e| e.to_string())?;
        Ok(categories)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_category(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Category>> {
        let repository = repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(category)
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
    async fn list_locations(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Location>> {
        let repository = repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let locations = repository.list().await.map_err(|e| e.to_string())?;
        Ok(locations)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_location(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Location>> {
        let repository = repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(location)
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
    async fn list_items(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let items = repository.list().await.map_err(|e| e.to_string())?;
        Ok(items)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_item(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(item)
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
    async fn list_collections(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Collection>> {
        let repository = repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collections = repository.list().await.map_err(|e| e.to_string())?;
        Ok(collections)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_collection(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Collection>> {
        let repository = repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(collection)
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
    async fn list_media(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Media>> {
        let repository = repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository.list().await.map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_media(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Media>> {
        let repository = repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository.find(id).await.map_err(|e| e.to_string())?;
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
    async fn list_publishes(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Publish>> {
        let repository = repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publishes = repository.list().await.map_err(|e| e.to_string())?;
        Ok(publishes)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_publish(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Publish>> {
        let repository = repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(Some(publish))
    }
}
