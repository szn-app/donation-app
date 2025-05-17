use crate::database::model::{ItemIntentAction, Pledge, PledgeStatus};
use crate::database::sql::{
    ADD_PLEDGE, GET_PLEDGES, GET_PLEDGES_BY_ITEM, GET_PLEDGES_BY_PROFILE, GET_PLEDGE_BY_ID,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;

pub struct PledgeRepository {
    pool: Pool,
}

impl PledgeRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_pledges(&self) -> Result<Vec<Pledge>> {
        debug!("Getting all pledges");
        let client = self.pool.get().await?;
        let rows = client.query(GET_PLEDGES, &[]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn get_pledge_by_id(&self, id: i64) -> Result<Option<Pledge>> {
        debug!("Getting pledge by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_PLEDGE_BY_ID, &[&id]).await?;
        Ok(row.map(Pledge::from))
    }

    pub async fn get_pledges_by_item(&self, id_item: i64) -> Result<Vec<Pledge>> {
        debug!("Getting pledges by item: {}", id_item);
        let client = self.pool.get().await?;
        let rows = client.query(GET_PLEDGES_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn get_pledges_by_profile(&self, id_profile: i64) -> Result<Vec<Pledge>> {
        debug!("Getting pledges by profile: {}", id_profile);
        let client = self.pool.get().await?;
        let rows = client.query(GET_PLEDGES_BY_PROFILE, &[&id_profile]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn add_pledge(
        &self,
        id_profile: i64,
        id_item: i64,
        intent_action: ItemIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge> {
        debug!(
            "Adding pledge for item: {} by profile: {}",
            id_item, id_profile
        );
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_PLEDGE,
                &[&id_profile, &id_item, &intent_action, &message, &status],
            )
            .await?;
        Ok(Pledge::from(row))
    }
}
