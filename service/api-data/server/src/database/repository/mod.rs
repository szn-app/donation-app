use crate::database::model;
use crate::database::sql;
use crate::server::connection::{self, postgresql::client, PostgresPool};
use deadpool_postgres::{Client, PoolError};
use log;
use std::error::Error;
use tokio_postgres;
use uuid::Uuid;

pub mod interaction;
pub mod listing;
pub mod test;
pub mod user;

pub use interaction::*;
pub use listing::*;
pub use test::*;
pub use user::*;

mod util {
    use super::*;

    fn log_full_db_err(err: &tokio_postgres::error::Error, msg: &str) {
        let dberr = match err.as_db_error() {
            None => {
                log::error!("Error unwrapping tokio_postgres DbError: {:?}", &err);
                return;
            }
            Some(err) => err,
        };
        log::error!(
            "DB error: {} {}",
            dberr.message(),
            dberr
                .detail()
                .expect("cannot retrieve detail error from postgres")
        );
        log::error!("{}", msg);
    }
}
