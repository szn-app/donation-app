use crate::database::model::interaction::{Pledge, PledgeIntentAction, PledgeStatus};
use crate::database::sql::interaction::pledge::{
    CREATE_PLEDGE, LIST_PLEDGES, FIND_PLEDGES_BY_ITEM, FIND_PLEDGES_BY_PROFILE, FIND_PLEDGE,
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

    pub async fn list(&self) -> Result<Vec<Pledge>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all pledges");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_PLEDGES, &[]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn find(
        &self,
        id: i64,
    ) -> Result<Option<Pledge>, Box<dyn Error + Send + Sync>> {
        debug!("Getting pledge by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_PLEDGE, &[&id]).await?;
        Ok(row.map(Pledge::from))
    }

    pub async fn find_by_item(
        &self,
        id_item: i64,
    ) -> Result<Vec<Pledge>, Box<dyn Error + Send + Sync>> {
        debug!("Getting pledges by item: {}", id_item);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_PLEDGES_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn find_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Pledge>, Box<dyn Error + Send + Sync>> {
        debug!("Getting pledges by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_PLEDGES_BY_PROFILE, &[&id_profile]).await?;
        Ok(rows.into_iter().map(Pledge::from).collect())
    }

    pub async fn create(
        &self,
        id_profile: Uuid,
        id_item: i64,
        intent_action: PledgeIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge, Box<dyn Error + Send + Sync>> {
        debug!("Adding pledge for item: {}", id_item);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_PLEDGE,
                &[&id_profile, &id_item, &intent_action, &message, &status],
            )
            .await?;
        Ok(Pledge::from(row))
    }

    pub async fn update(
        &self,
        id: i64,
        status: PledgeStatus,
    ) -> Result<Pledge, Box<dyn Error + Send + Sync>> {
        debug!("Updating pledge: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client.query_one(UPDATE_PLEDGE, &[&id, &status]).await?;
        Ok(Pledge::from(row))
    }
}
