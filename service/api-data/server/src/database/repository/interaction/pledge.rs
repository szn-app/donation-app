use crate::database::model::interaction::{Pledge, PledgeStatus};
use crate::database::model::listing::ItemIntentAction;
use crate::database::sql::interaction::pledge::{
    ADD_PLEDGE, GET_PLEDGES, GET_PLEDGES_BY_ITEM, GET_PLEDGES_BY_PROFILE, GET_PLEDGE_BY_ID,
    UPDATE_PLEDGE,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct PledgeRepository {
    pool: PostgresPool,
}

impl PledgeRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_pledges(&self) -> Result<Vec<Pledge>, Box<dyn Error>> {
        debug!("Getting all pledges");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PLEDGES, &[]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn get_pledge_by_id(&self, id: i64) -> Result<Option<Pledge>, Box<dyn Error>> {
        debug!("Getting pledge by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_PLEDGE_BY_ID, &[&id]).await?;
        Ok(row.map(Pledge::from))
    }

    pub async fn get_pledges_by_item(&self, id_item: i64) -> Result<Vec<Pledge>, Box<dyn Error>> {
        debug!("Getting pledges by item: {}", id_item);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PLEDGES_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn get_pledges_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Pledge>, Box<dyn Error>> {
        debug!("Getting pledges by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PLEDGES_BY_PROFILE, &[&id_profile]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn add_pledge(
        &self,
        id_profile: Uuid,
        id_item: i64,
        intent_action: ItemIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge, Box<dyn Error>> {
        debug!("Adding pledge for item: {}", id_item);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_PLEDGE,
                &[&id_profile, &id_item, &intent_action, &message, &status],
            )
            .await?;
        Ok(Pledge::from(row))
    }

    pub async fn update_pledge(
        &self,
        id: i64,
        status: PledgeStatus,
    ) -> Result<Pledge, Box<dyn Error>> {
        debug!("Updating pledge: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client.query_one(UPDATE_PLEDGE, &[&id, &status]).await?;
        Ok(Pledge::from(row))
    }
}
