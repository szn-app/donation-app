use crate::database::model;
use crate::database::sql;
use crate::server::connection::{self, postgresql::client};
use deadpool_postgres::{Client, Pool};
use log;
use std::error::Error;
use tokio_postgres;
use uuid::Uuid;

pub mod user {
    use super::*;

    pub struct AccountRepository;

    impl AccountRepository {
        pub async fn get_accounts(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Account>, Box<dyn Error>> {
            log::debug!("--> get_accounts");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_ACCOUNTS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Iterate over the rows and convert each Row into an Account
            let accounts: Vec<model::user::Account> = rows
                .into_iter()
                .map(|row| row.into()) // Use the From<Row> for Account implementation
                .collect();

            Ok(accounts)
        }

        pub async fn add_account(
            postgres_pool_group: &connection::PostgresPool,
            id: uuid::Uuid,
        ) -> Result<model::user::Account, Box<dyn Error>> {
            log::debug!("--> add_account");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            // Insert user into database
            let row: tokio_postgres::Row = client
                .query_one(sql::ADD_ACCOUNT, &[&id])
                .await
                .map_err(|e| {
                    log::error!("Database error while adding user: {}", e);
                    e
                })?;

            let account: model::user::Account = row.into();

            Ok(account)
        }

        pub async fn get_account_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: uuid::Uuid,
        ) -> Result<model::user::Account, Box<dyn Error>> {
            log::debug!("--> get_account_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row =
                client
                    .query_one(sql::GET_ACCOUNT_BY_ID, &[&id])
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let account: model::user::Account = row.into();

            Ok(account)
        }
    }

    pub struct ProfileRepository;

    impl ProfileRepository {
        pub async fn get_profiles(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Profile>, Box<dyn Error>> {
            log::debug!("--> get_profiles");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_PROFILES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let profiles: Vec<model::user::Profile> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(profiles)
        }

        pub async fn get_profile_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::user::Profile, Box<dyn Error>> {
            log::debug!("--> get_profile_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row =
                client
                    .query_one(sql::GET_PROFILE_BY_ID, &[&id])
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let profile: model::user::Profile = row.into();

            Ok(profile)
        }

        pub async fn add_profile(
            postgres_pool_group: &connection::PostgresPool,
            profile: model::user::Profile,
        ) -> Result<model::user::Profile, Box<dyn Error>> {
            log::debug!("--> add_profile");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_PROFILE,
                    &[
                        &profile.name,
                        &profile.description,
                        &profile.type_,
                        &profile.owner,
                        &profile.created_by,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding profile: {}", e);
                    e
                })?;

            let profile: model::user::Profile = row.into();

            Ok(profile)
        }

        pub async fn update_profile(
            postgres_pool_group: &connection::PostgresPool,
            profile: model::user::Profile,
        ) -> Result<model::user::Profile, Box<dyn Error>> {
            log::debug!("--> update_profile");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_PROFILE,
                    &[
                        &profile.id,
                        &profile.name,
                        &profile.description,
                        &profile.type_,
                        &profile.updated_at,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating profile: {}", e);
                    e
                })?;

            let profile: model::user::Profile = row.into();

            Ok(profile)
        }
    }

    pub struct CommunityRepository;

    impl CommunityRepository {
        pub async fn get_communities(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Community>, Box<dyn Error>> {
            log::debug!("--> get_communities");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_COMMUNITIES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let communities: Vec<model::user::Community> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(communities)
        }

        pub async fn get_community_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::user::Community, Box<dyn Error>> {
            log::debug!("--> get_community_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_COMMUNITY_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let community: model::user::Community = row.into();

            Ok(community)
        }

        pub async fn add_community(
            postgres_pool_group: &connection::PostgresPool,
            community: model::user::Community,
        ) -> Result<model::user::Community, Box<dyn Error>> {
            log::debug!("--> add_community");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_COMMUNITY,
                    &[
                        &community.title,
                        &community.description,
                        &community.type_,
                        &community.owner,
                        &community.created_by,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding community: {}", e);
                    e
                })?;

            let community: model::user::Community = row.into();

            Ok(community)
        }

        pub async fn update_community(
            postgres_pool_group: &connection::PostgresPool,
            community: model::user::Community,
        ) -> Result<model::user::Community, Box<dyn Error>> {
            log::debug!("--> update_community");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_COMMUNITY,
                    &[
                        &community.id,
                        &community.title,
                        &community.description,
                        &community.type_,
                        &community.updated_at,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating community: {}", e);
                    e
                })?;

            let community: model::user::Community = row.into();

            Ok(community)
        }
    }

    pub struct CommitteeRepository;

    impl CommitteeRepository {
        pub async fn get_committees(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Committee>, Box<dyn Error>> {
            log::debug!("--> get_committees");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_COMMITTEES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let committees: Vec<model::user::Committee> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(committees)
        }

        pub async fn get_committee_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id_profile: i64,
            id_community: i64,
        ) -> Result<model::user::Committee, Box<dyn Error>> {
            log::debug!("--> get_committee_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_COMMITTEE_BY_ID, &[&id_profile, &id_community])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let committee: model::user::Committee = row.into();

            Ok(committee)
        }

        pub async fn add_committee(
            postgres_pool_group: &connection::PostgresPool,
            committee: model::user::Committee,
        ) -> Result<model::user::Committee, Box<dyn Error>> {
            log::debug!("--> add_committee");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_COMMITTEE,
                    &[
                        &committee.id_profile,
                        &committee.id_community,
                        &committee.member_role,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding committee: {}", e);
                    e
                })?;

            let committee: model::user::Committee = row.into();

            Ok(committee)
        }

        pub async fn update_committee(
            postgres_pool_group: &connection::PostgresPool,
            committee: model::user::Committee,
        ) -> Result<model::user::Committee, Box<dyn Error>> {
            log::debug!("--> update_committee");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_COMMITTEE,
                    &[
                        &committee.id_profile,
                        &committee.id_community,
                        &committee.member_role,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating committee: {}", e);
                    e
                })?;

            let committee: model::user::Committee = row.into();

            Ok(committee)
        }
    }
}

pub mod listing {
    use super::*;

    pub struct CategoryRepository;

    impl CategoryRepository {
        pub async fn get_categories(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Category>, Box<dyn Error>> {
            log::debug!("--> get_categories");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_CATEGORIES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let categories: Vec<model::listing::Category> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(categories)
        }

        pub async fn get_category_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::listing::Category, Box<dyn Error>> {
            log::debug!("--> get_category_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_CATEGORY_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let category: model::listing::Category = row.into();

            Ok(category)
        }

        pub async fn add_category(
            postgres_pool_group: &connection::PostgresPool,
            category: model::listing::Category,
        ) -> Result<model::listing::Category, Box<dyn Error>> {
            log::debug!("--> add_category");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_CATEGORY,
                    &[
                        &category.title,
                        &category.description,
                        &category.category_parent,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding category: {}", e);
                    e
                })?;

            let category: model::listing::Category = row.into();

            Ok(category)
        }

        pub async fn update_category(
            postgres_pool_group: &connection::PostgresPool,
            category: model::listing::Category,
        ) -> Result<model::listing::Category, Box<dyn Error>> {
            log::debug!("--> update_category");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_CATEGORY,
                    &[
                        &category.id,
                        &category.title,
                        &category.description,
                        &category.category_parent,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating category: {}", e);
                    e
                })?;

            let category: model::listing::Category = row.into();

            Ok(category)
        }
    }

    pub struct LocationRepository;

    impl LocationRepository {
        pub async fn get_locations(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Location>, Box<dyn Error>> {
            log::debug!("--> get_locations");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_LOCATIONS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let locations: Vec<model::listing::Location> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(locations)
        }

        pub async fn get_location_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::listing::Location, Box<dyn Error>> {
            log::debug!("--> get_location_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_LOCATION_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let location: model::listing::Location = row.into();

            Ok(location)
        }

        pub async fn add_location(
            postgres_pool_group: &connection::PostgresPool,
            location: model::listing::Location,
        ) -> Result<model::listing::Location, Box<dyn Error>> {
            log::debug!("--> add_location");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_LOCATION,
                    &[
                        &location.address_line1,
                        &location.address_line2,
                        &location.city,
                        &location.state,
                        &location.district,
                        &location.country,
                        &location.geom,
                        &location.entrance_note,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding location: {}", e);
                    e
                })?;

            let location: model::listing::Location = row.into();

            Ok(location)
        }

        pub async fn update_location(
            postgres_pool_group: &connection::PostgresPool,
            location: model::listing::Location,
        ) -> Result<model::listing::Location, Box<dyn Error>> {
            log::debug!("--> update_location");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_LOCATION,
                    &[
                        &location.id,
                        &location.address_line1,
                        &location.address_line2,
                        &location.city,
                        &location.state,
                        &location.district,
                        &location.country,
                        &location.geom,
                        &location.entrance_note,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating location: {}", e);
                    e
                })?;

            let location: model::listing::Location = row.into();

            Ok(location)
        }
    }

    pub struct ItemRepository;

    impl ItemRepository {
        pub async fn get_items(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Item>, Box<dyn Error>> {
            log::debug!("--> get_items");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_ITEMS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let items: Vec<model::listing::Item> = rows.into_iter().map(|row| row.into()).collect();

            Ok(items)
        }

        pub async fn get_item_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::listing::Item, Box<dyn Error>> {
            log::debug!("--> get_item_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_ITEM_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let item: model::listing::Item = row.into();

            Ok(item)
        }
    }

    pub struct CollectionRepository;

    impl CollectionRepository {
        pub async fn get_collections(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Collection>, Box<dyn Error>> {
            log::debug!("--> get_collections");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_COLLECTIONS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let collections: Vec<model::listing::Collection> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(collections)
        }

        pub async fn get_collection_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::listing::Collection, Box<dyn Error>> {
            log::debug!("--> get_collection_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_COLLECTION_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let collection: model::listing::Collection = row.into();

            Ok(collection)
        }

        pub async fn get_collections_by_community(
            postgres_pool_group: &connection::PostgresPool,
            id_community: i64,
        ) -> Result<Vec<model::listing::Collection>, Box<dyn Error>> {
            log::debug!("--> get_collections_by_community");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_COLLECTIONS_BY_COMMUNITY, &[&id_community])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let collections: Vec<model::listing::Collection> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(collections)
        }

        pub async fn add_collection(
            postgres_pool_group: &connection::PostgresPool,
            collection: model::listing::Collection,
        ) -> Result<model::listing::Collection, Box<dyn Error>> {
            log::debug!("--> add_collection");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_COLLECTION,
                    &[
                        &collection.id_community,
                        &collection.title,
                        &collection.visibility,
                        &collection.type_,
                        &collection.position,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding collection: {}", e);
                    e
                })?;

            let collection: model::listing::Collection = row.into();

            Ok(collection)
        }

        pub async fn update_collection(
            postgres_pool_group: &connection::PostgresPool,
            collection: model::listing::Collection,
        ) -> Result<model::listing::Collection, Box<dyn Error>> {
            log::debug!("--> update_collection");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_COLLECTION,
                    &[
                        &collection.id,
                        &collection.id_community,
                        &collection.title,
                        &collection.visibility,
                        &collection.type_,
                        &collection.position,
                        &collection.updated_at,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating collection: {}", e);
                    e
                })?;

            let collection: model::listing::Collection = row.into();

            Ok(collection)
        }
    }

    pub struct MediaRepository;

    impl MediaRepository {
        pub async fn get_media(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Media>, Box<dyn Error>> {
            log::debug!("--> get_media");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_MEDIA, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let media: Vec<model::listing::Media> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(media)
        }

        pub async fn get_media_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::listing::Media, Box<dyn Error>> {
            log::debug!("--> get_media_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_MEDIA_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let media: model::listing::Media = row.into();

            Ok(media)
        }

        pub async fn get_media_by_item(
            postgres_pool_group: &connection::PostgresPool,
            id_item: i64,
        ) -> Result<Vec<model::listing::Media>, Box<dyn Error>> {
            log::debug!("--> get_media_by_item");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_MEDIA_BY_ITEM, &[&id_item])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let media: Vec<model::listing::Media> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(media)
        }

        pub async fn add_media(
            postgres_pool_group: &connection::PostgresPool,
            media: model::listing::Media,
        ) -> Result<model::listing::Media, Box<dyn Error>> {
            log::debug!("--> add_media");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_MEDIA,
                    &[&media.id_item, &media.caption, &media.url, &media.type_],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding media: {}", e);
                    e
                })?;

            let media: model::listing::Media = row.into();

            Ok(media)
        }

        pub async fn delete_media(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<(), Box<dyn Error>> {
            log::debug!("--> delete_media");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            client
                .execute(sql::DELETE_MEDIA, &[&id])
                .await
                .map_err(|e| {
                    log::error!("Database error while deleting media: {}", e);
                    e
                })?;

            Ok(())
        }
    }

    pub struct PublishRepository;

    impl PublishRepository {
        pub async fn get_publishes(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::listing::Publish>, Box<dyn Error>> {
            log::debug!("--> get_publishes");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_PUBLISHES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let publishes: Vec<model::listing::Publish> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(publishes)
        }

        pub async fn get_publish_by_item_and_collection(
            postgres_pool_group: &connection::PostgresPool,
            id_item: i64,
            id_collection: i64,
        ) -> Result<model::listing::Publish, Box<dyn Error>> {
            log::debug!("--> get_publish_by_item_and_collection");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::GET_PUBLISH_BY_ITEM_AND_COLLECTION,
                    &[&id_item, &id_collection],
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let publish: model::listing::Publish = row.into();

            Ok(publish)
        }

        pub async fn get_publishes_by_collection(
            postgres_pool_group: &connection::PostgresPool,
            id_collection: i64,
        ) -> Result<Vec<model::listing::Publish>, Box<dyn Error>> {
            log::debug!("--> get_publishes_by_collection");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_PUBLISHES_BY_COLLECTION, &[&id_collection])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let publishes: Vec<model::listing::Publish> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(publishes)
        }

        pub async fn get_publishes_by_item(
            postgres_pool_group: &connection::PostgresPool,
            id_item: i64,
        ) -> Result<Vec<model::listing::Publish>, Box<dyn Error>> {
            log::debug!("--> get_publishes_by_item");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_PUBLISHES_BY_ITEM, &[&id_item])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let publishes: Vec<model::listing::Publish> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(publishes)
        }

        pub async fn add_publish(
            postgres_pool_group: &connection::PostgresPool,
            publish: model::listing::Publish,
        ) -> Result<model::listing::Publish, Box<dyn Error>> {
            log::debug!("--> add_publish");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_PUBLISH,
                    &[
                        &publish.id_item,
                        &publish.id_collection,
                        &publish.note,
                        &publish.position,
                        &publish.added_by,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding publish: {}", e);
                    e
                })?;

            let publish: model::listing::Publish = row.into();

            Ok(publish)
        }

        pub async fn update_publish(
            postgres_pool_group: &connection::PostgresPool,
            publish: model::listing::Publish,
        ) -> Result<model::listing::Publish, Box<dyn Error>> {
            log::debug!("--> update_publish");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_PUBLISH,
                    &[
                        &publish.id_item,
                        &publish.id_collection,
                        &publish.note,
                        &publish.position,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating publish: {}", e);
                    e
                })?;

            let publish: model::listing::Publish = row.into();

            Ok(publish)
        }

        pub async fn delete_publish(
            postgres_pool_group: &connection::PostgresPool,
            id_item: i64,
            id_collection: i64,
        ) -> Result<(), Box<dyn Error>> {
            log::debug!("--> delete_publish");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            client
                .execute(sql::DELETE_PUBLISH, &[&id_item, &id_collection])
                .await
                .map_err(|e| {
                    log::error!("Database error while deleting publish: {}", e);
                    e
                })?;

            Ok(())
        }
    }
}

pub mod test {
    use super::*;

    pub struct TestRepository;

    impl TestRepository {
        pub async fn get_tests(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::test::Test>, Box<dyn Error>> {
            log::debug!("--> get_tests");

            // get connection from pool (single attempt)
            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_TESTS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Iterate over the rows and convert each Row into an Account
            let tests: Vec<model::test::Test> = rows
                .into_iter()
                .map(|row| row.into()) // Use the From<Row> for Account implementation
                .collect();

            Ok(tests)
        }
    }
}

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

pub mod interaction {
    use super::*;

    pub struct ScheduleRepository;

    impl ScheduleRepository {
        pub async fn get_schedules(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::Schedule>, Box<dyn Error>> {
            log::debug!("--> get_schedules");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_SCHEDULES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let schedules: Vec<model::interaction::Schedule> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(schedules)
        }

        pub async fn get_schedule_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::interaction::Schedule, Box<dyn Error>> {
            log::debug!("--> get_schedule_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_SCHEDULE_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let schedule: model::interaction::Schedule = row.into();

            Ok(schedule)
        }

        pub async fn add_schedule(
            postgres_pool_group: &connection::PostgresPool,
            schedule: model::interaction::Schedule,
        ) -> Result<model::interaction::Schedule, Box<dyn Error>> {
            log::debug!("--> add_schedule");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(sql::ADD_SCHEDULE, &[&schedule.scheduled_for])
                .await
                .map_err(|e| {
                    log::error!("Database error while adding schedule: {}", e);
                    e
                })?;

            let schedule: model::interaction::Schedule = row.into();

            Ok(schedule)
        }
    }

    pub struct PledgeRepository;

    impl PledgeRepository {
        pub async fn get_pledges(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::Pledge>, Box<dyn Error>> {
            log::debug!("--> get_pledges");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_PLEDGES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let pledges: Vec<model::interaction::Pledge> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(pledges)
        }

        pub async fn get_pledge_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::interaction::Pledge, Box<dyn Error>> {
            log::debug!("--> get_pledge_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_PLEDGE_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let pledge: model::interaction::Pledge = row.into();

            Ok(pledge)
        }

        pub async fn add_pledge(
            postgres_pool_group: &connection::PostgresPool,
            pledge: model::interaction::Pledge,
        ) -> Result<model::interaction::Pledge, Box<dyn Error>> {
            log::debug!("--> add_pledge");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_PLEDGE,
                    &[
                        &pledge.id_profile,
                        &pledge.id_item,
                        &pledge.intent_action,
                        &pledge.message,
                        &pledge.status,
                        &pledge.updated_by,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding pledge: {}", e);
                    e
                })?;

            let pledge: model::interaction::Pledge = row.into();

            Ok(pledge)
        }

        pub async fn update_pledge(
            postgres_pool_group: &connection::PostgresPool,
            pledge: model::interaction::Pledge,
        ) -> Result<model::interaction::Pledge, Box<dyn Error>> {
            log::debug!("--> update_pledge");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_PLEDGE,
                    &[
                        &pledge.id,
                        &pledge.status,
                        &pledge.updated_at,
                        &pledge.updated_by,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating pledge: {}", e);
                    e
                })?;

            let pledge: model::interaction::Pledge = row.into();

            Ok(pledge)
        }
    }

    pub struct TransactionRepository;

    impl TransactionRepository {
        pub async fn get_transactions(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::Transaction>, Box<dyn Error>> {
            log::debug!("--> get_transactions");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_TRANSACTIONS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let transactions: Vec<model::interaction::Transaction> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(transactions)
        }

        pub async fn get_transaction_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::interaction::Transaction, Box<dyn Error>> {
            log::debug!("--> get_transaction_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_TRANSACTION_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let transaction: model::interaction::Transaction = row.into();

            Ok(transaction)
        }

        pub async fn add_transaction(
            postgres_pool_group: &connection::PostgresPool,
            transaction: model::interaction::Transaction,
        ) -> Result<model::interaction::Transaction, Box<dyn Error>> {
            log::debug!("--> add_transaction");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_TRANSACTION,
                    &[
                        &transaction.id_pledge,
                        &transaction.status,
                        &transaction.id_schedule,
                        &transaction.id_location,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding transaction: {}", e);
                    e
                })?;

            let transaction: model::interaction::Transaction = row.into();

            Ok(transaction)
        }

        pub async fn update_transaction(
            postgres_pool_group: &connection::PostgresPool,
            transaction: model::interaction::Transaction,
        ) -> Result<model::interaction::Transaction, Box<dyn Error>> {
            log::debug!("--> update_transaction");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_TRANSACTION,
                    &[
                        &transaction.id,
                        &transaction.status,
                        &transaction.id_schedule,
                        &transaction.id_location,
                        &transaction.updated_at,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating transaction: {}", e);
                    e
                })?;

            let transaction: model::interaction::Transaction = row.into();

            Ok(transaction)
        }
    }

    pub struct MessageRepository;

    impl MessageRepository {
        pub async fn get_messages(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::Message>, Box<dyn Error>> {
            log::debug!("--> get_messages");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_MESSAGES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let messages: Vec<model::interaction::Message> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(messages)
        }

        pub async fn get_message_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::interaction::Message, Box<dyn Error>> {
            log::debug!("--> get_message_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row =
                client
                    .query_one(sql::GET_MESSAGE_BY_ID, &[&id])
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let message: model::interaction::Message = row.into();

            Ok(message)
        }

        pub async fn add_message(
            postgres_pool_group: &connection::PostgresPool,
            message: model::interaction::Message,
        ) -> Result<model::interaction::Message, Box<dyn Error>> {
            log::debug!("--> add_message");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_MESSAGE,
                    &[
                        &message.id_sender,
                        &message.id_transaction,
                        &message.type_,
                        &message.content,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding message: {}", e);
                    e
                })?;

            let message: model::interaction::Message = row.into();

            Ok(message)
        }

        pub async fn update_message(
            postgres_pool_group: &connection::PostgresPool,
            message: model::interaction::Message,
        ) -> Result<model::interaction::Message, Box<dyn Error>> {
            log::debug!("--> update_message");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_MESSAGE,
                    &[&message.id, &message.content, &message.updated_at],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating message: {}", e);
                    e
                })?;

            let message: model::interaction::Message = row.into();

            Ok(message)
        }
    }

    pub struct ReviewRepository;

    impl ReviewRepository {
        pub async fn get_reviews(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::Review>, Box<dyn Error>> {
            log::debug!("--> get_reviews");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_REVIEWS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let reviews: Vec<model::interaction::Review> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(reviews)
        }

        pub async fn get_review_by_transaction_and_subject(
            postgres_pool_group: &connection::PostgresPool,
            id_transaction: i64,
            id_subject_profile: i64,
        ) -> Result<model::interaction::Review, Box<dyn Error>> {
            log::debug!("--> get_review_by_transaction_and_subject");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::GET_REVIEW_BY_TRANSACTION_AND_SUBJECT,
                    &[&id_transaction, &id_subject_profile],
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let review: model::interaction::Review = row.into();

            Ok(review)
        }

        pub async fn add_review(
            postgres_pool_group: &connection::PostgresPool,
            review: model::interaction::Review,
        ) -> Result<model::interaction::Review, Box<dyn Error>> {
            log::debug!("--> add_review");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_REVIEW,
                    &[
                        &review.id_transaction,
                        &review.id_subject_profile,
                        &review.reviewer,
                        &review.comment,
                        &review.score,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding review: {}", e);
                    e
                })?;

            let review: model::interaction::Review = row.into();

            Ok(review)
        }
    }

    pub struct ScheduleOpportunityRepository;

    impl ScheduleOpportunityRepository {
        pub async fn get_schedule_opportunities(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::interaction::ScheduleOpportunity>, Box<dyn Error>> {
            log::debug!("--> get_schedule_opportunities");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_SCHEDULE_OPPORTUNITIES, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let opportunities: Vec<model::interaction::ScheduleOpportunity> =
                rows.into_iter().map(|row| row.into()).collect();

            Ok(opportunities)
        }

        pub async fn get_schedule_opportunity_by_id(
            postgres_pool_group: &connection::PostgresPool,
            id: i64,
        ) -> Result<model::interaction::ScheduleOpportunity, Box<dyn Error>> {
            log::debug!("--> get_schedule_opportunity_by_id");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let row: tokio_postgres::Row = client
                .query_one(sql::GET_SCHEDULE_OPPORTUNITY_BY_ID, &[&id])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let opportunity: model::interaction::ScheduleOpportunity = row.into();

            Ok(opportunity)
        }

        pub async fn add_schedule_opportunity(
            postgres_pool_group: &connection::PostgresPool,
            opportunity: model::interaction::ScheduleOpportunity,
        ) -> Result<model::interaction::ScheduleOpportunity, Box<dyn Error>> {
            log::debug!("--> add_schedule_opportunity");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::ADD_SCHEDULE_OPPORTUNITY,
                    &[&opportunity.window_start, &opportunity.window_end],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while adding schedule opportunity: {}", e);
                    e
                })?;

            let opportunity: model::interaction::ScheduleOpportunity = row.into();

            Ok(opportunity)
        }

        pub async fn update_schedule_opportunity(
            postgres_pool_group: &connection::PostgresPool,
            opportunity: model::interaction::ScheduleOpportunity,
        ) -> Result<model::interaction::ScheduleOpportunity, Box<dyn Error>> {
            log::debug!("--> update_schedule_opportunity");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            let row: tokio_postgres::Row = client
                .query_one(
                    sql::UPDATE_SCHEDULE_OPPORTUNITY,
                    &[
                        &opportunity.id,
                        &opportunity.window_start,
                        &opportunity.window_end,
                    ],
                )
                .await
                .map_err(|e| {
                    log::error!("Database error while updating schedule opportunity: {}", e);
                    e
                })?;

            let opportunity: model::interaction::ScheduleOpportunity = row.into();

            Ok(opportunity)
        }
    }
}
