use crate::api::graphql::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

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
    async fn add_category(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> FieldResult<model::Category> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .add_category(&name, &description, parent_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_category(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> FieldResult<model::Category> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .update_category(id, name, description, parent_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_category(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository =
            repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_category(id)
            .await
            .map_err(|e| e.to_string())?;
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
    async fn add_location(
        &self,
        ctx: &Context<'_>,
        name: String,
        address: String,
        city: String,
        state: String,
        country: String,
        postal_code: String,
        id_profile: uuid::Uuid,
    ) -> FieldResult<model::Location> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository
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
            .map_err(|e| e.to_string())?;
        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_location(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
    ) -> FieldResult<model::Location> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository
            .update_location(id, name, address, city, state, country, postal_code)
            .await
            .map_err(|e| e.to_string())?;
        Ok(location)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_location(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository =
            repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_location(id)
            .await
            .map_err(|e| e.to_string())?;
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
    async fn add_item(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: String,
        id_category: i64,
        id_profile: uuid::Uuid,
        id_location: Option<i64>,
        price: Option<f64>,
        currency: Option<String>,
    ) -> FieldResult<model::Item> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .add_item(
                &title,
                &description,
                id_category,
                id_profile,
                id_location,
                price,
                currency,
            )
            .await
            .map_err(|e| e.to_string())?;
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
        id_category: Option<i64>,
        id_location: Option<i64>,
        price: Option<f64>,
        currency: Option<String>,
    ) -> FieldResult<model::Item> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository
            .update_item(
                id,
                title,
                description,
                id_category,
                id_location,
                price,
                currency,
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(item)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_item(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_item(id)
            .await
            .map_err(|e| e.to_string())?;
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
    async fn add_collection(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        id_profile: uuid::Uuid,
        is_public: bool,
    ) -> FieldResult<model::Collection> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .add_collection(&name, &description, id_profile, is_public)
            .await
            .map_err(|e| e.to_string())?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_collection(
        &self,
        ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> FieldResult<model::Collection> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .update_collection(id, name, description, is_public)
            .await
            .map_err(|e| e.to_string())?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_collection(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository =
            repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_collection(id)
            .await
            .map_err(|e| e.to_string())?;
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
    async fn add_media(
        &self,
        ctx: &Context<'_>,
        id_item: i64,
        url: String,
        media_type: String,
        position: i32,
    ) -> FieldResult<model::Media> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .add_media(id_item, &url, &media_type, position)
            .await
            .map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_media(
        &self,
        ctx: &Context<'_>,
        id: i64,
        url: Option<String>,
        media_type: Option<String>,
        position: Option<i32>,
    ) -> FieldResult<model::Media> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .update_media(id, url, media_type, position)
            .await
            .map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_media(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository =
            repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_media(id)
            .await
            .map_err(|e| e.to_string())?;
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
    async fn add_publish(
        &self,
        ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        created_by: uuid::Uuid,
    ) -> FieldResult<model::Publish> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .add_publish(id_item, id_collection, note, position, created_by)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_publish(
        &self,
        ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        updated_by: uuid::Uuid,
    ) -> FieldResult<model::Publish> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository
            .update_publish(id_item, id_collection, note, position, updated_by)
            .await
            .map_err(|e| e.to_string())?;
        Ok(publish)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_publish(
        &self,
        ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
    ) -> FieldResult<bool> {
        let repository =
            repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_publish(id_item, id_collection)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
    }
}
