use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::Location;
use crate::database::repository::listing::location::LocationRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use uuid::Uuid;

pub struct LocationMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl LocationMutation {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    pub async fn create_location(
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

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
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
            .update(id, name, address, city, state, country, postal_code)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(location)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_location(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
} 