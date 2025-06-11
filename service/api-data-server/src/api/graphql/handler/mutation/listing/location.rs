use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{Location, CoordinatesInput, GeoPoint};
use crate::database::repository::listing::location::LocationRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, Object, Result};

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
        address_line1: String,
        address_line2: Option<String>,
        city: String,
        state: Option<String>,
        district: Option<String>,
        country: String,
        coordinates: Option<CoordinatesInput>,
        entrance_note: Option<String>,
    ) -> Result<Location> {
        let location_repository = LocationRepository::new(self.postgres_pool_group.clone());
        
        // Convert coordinates to GeoPoint if present
        let geom = coordinates.map(GeoPoint::from);

        let location = location_repository
            .create(
                address_line1,
                address_line2,
                city,
                state.unwrap_or_default(),
                district,
                country,
                geom,
                entrance_note,
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
        address_line1: Option<String>,
        address_line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        district: Option<String>,
        country: Option<String>,
        coordinates: Option<CoordinatesInput>,
        entrance_note: Option<String>,
    ) -> Result<Location> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());

        // Convert coordinates to GeoPoint if present
        let geom = coordinates.map(GeoPoint::from);

        let location = repository
            .update(
                id,
                address_line1,
                address_line2,
                city,
                state,
                district,
                country,
                geom,
                entrance_note,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(location)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_location(&self, _ctx: &Context<'_>, id: i64) -> Result<bool> {
        let repository = LocationRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}
