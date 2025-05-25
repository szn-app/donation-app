// SQL queries for database operations
// Each module contains parametrized queries that tokio-postgres uses to create PREPARE statements

pub mod interaction;
pub mod listing;
pub mod test;
pub mod user;

// Re-export commonly used queries
// Note: These are organized by domain and operation type

// User domain
pub use user::account::{
    CREATE_ACCOUNT,
    LIST_ACCOUNTS,
    FIND_ACCOUNT,
    UPDATE_ACCOUNT,
    DELETE_ACCOUNT,
};

pub use user::profile::{
    CREATE_PROFILE,
    LIST_PROFILES,
    FIND_PROFILE,
    UPDATE_PROFILE,
    DELETE_PROFILE,
};

pub use user::community::{
    CREATE_COMMUNITY,
    LIST_COMMUNITIES,
    FIND_COMMUNITY,
    UPDATE_COMMUNITY,
    DELETE_COMMUNITY,
};

pub use user::committee::{
    CREATE_COMMITTEE,
    LIST_COMMITTEES,
    FIND_COMMITTEES_BY_COMMUNITY,
    FIND_COMMITTEES_BY_PROFILE,
    FIND_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
    UPDATE_COMMITTEE_ROLE,
    DELETE_COMMITTEE,
};

// Interaction domain
pub use interaction::message::{
    CREATE_MESSAGE,
    LIST_MESSAGES,
    FIND_MESSAGES_BY_TRANSACTION,
    FIND_MESSAGE,
};

pub use interaction::pledge::{
    CREATE_PLEDGE,
    LIST_PLEDGES,
    FIND_PLEDGES_BY_ITEM,
    FIND_PLEDGES_BY_PROFILE,
    FIND_PLEDGE,
};

pub use interaction::review::{
    LIST_REVIEWS,
    FIND_REVIEW_BY_TRANSACTION_AND_SUBJECT,
};

pub use interaction::schedule::{
    CREATE_SCHEDULE,
    LIST_SCHEDULES,
    FIND_SCHEDULE,
    UPDATE_SCHEDULE,
};

pub use interaction::schedule_opportunity::{
    CREATE_SCHEDULE_OPPORTUNITY,
    LIST_SCHEDULE_OPPORTUNITIES,
    FIND_SCHEDULE_OPPORTUNITY,
    UPDATE_SCHEDULE_OPPORTUNITY,
};

pub use interaction::transaction::{
    CREATE_TRANSACTION,
    LIST_TRANSACTIONS,
    FIND_TRANSACTIONS_BY_PLEDGE,
    FIND_TRANSACTION,
    UPDATE_TRANSACTION,
};

// Listing domain
pub use listing::category::{
    CREATE_CATEGORY,
    LIST_CATEGORIES,
    FIND_CATEGORY,
    UPDATE_CATEGORY,
};

pub use listing::collection::{
    CREATE_COLLECTION,
    LIST_COLLECTIONS,
    FIND_COLLECTIONS_BY_PROFILE,
    FIND_COLLECTION,
    UPDATE_COLLECTION,
};

pub use listing::item::{
    LIST_ITEMS,
    FIND_ITEM,
};

pub use listing::location::{
    CREATE_LOCATION,
    LIST_LOCATIONS,
    FIND_LOCATION,
    UPDATE_LOCATION,
};

pub use listing::media::{
    CREATE_MEDIA,
    DELETE_MEDIA,
    LIST_MEDIA,
    FIND_MEDIA,
    FIND_MEDIA_BY_ITEM,
};

pub use listing::publish::{
    CREATE_PUBLISH,
    DELETE_PUBLISH,
    LIST_PUBLISHES,
    FIND_PUBLISHES_BY_COLLECTION,
    FIND_PUBLISHES_BY_ITEM,
    FIND_PUBLISH_BY_ITEM_AND_COLLECTION,
    UPDATE_PUBLISH,
};

// Test domain
pub use test::LIST_TESTS;
