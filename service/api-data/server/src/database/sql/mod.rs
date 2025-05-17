// raw Postgres SQL parametrized queries (which tokio-postgres creates PREPARE statements from and execution)

pub const GET_TESTS: &str = include_str!("./test/GET_TESTS.sql");

// Account schema queries
pub const GET_ACCOUNTS: &str = include_str!("./user/account/GET_ACCOUNTS.sql");
pub const ADD_ACCOUNT: &str = include_str!("./user/account/ADD_ACCOUNT.sql");
pub const GET_ACCOUNT_BY_ID: &str = include_str!("./user/account/GET_ACCOUNT_BY_ID.sql");

// Community related queries
pub const GET_COMMUNITIES: &str = include_str!("./user/community/GET_COMMUNITIES.sql");
pub const GET_COMMUNITY_BY_ID: &str = include_str!("./user/community/GET_COMMUNITY_BY_ID.sql");
pub const ADD_COMMUNITY: &str = include_str!("./user/community/ADD_COMMUNITY.sql");
pub const UPDATE_COMMUNITY: &str = include_str!("./user/community/UPDATE_COMMUNITY.sql");

// Profile related queries
pub const GET_PROFILES: &str = include_str!("./user/profile/GET_PROFILES.sql");
pub const GET_PROFILE_BY_ID: &str = include_str!("./user/profile/GET_PROFILE_BY_ID.sql");
pub const ADD_PROFILE: &str = include_str!("./user/profile/ADD_PROFILE.sql");
pub const UPDATE_PROFILE: &str = include_str!("./user/profile/UPDATE_PROFILE.sql");

// Committee related queries
pub const GET_COMMITTEES: &str = include_str!("./user/committee/GET_COMMITTEES.sql");
pub const GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY: &str =
    include_str!("./user/committee/GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY.sql");
pub const GET_COMMITTEES_BY_COMMUNITY: &str =
    include_str!("./user/committee/GET_COMMITTEES_BY_COMMUNITY.sql");
pub const GET_COMMITTEES_BY_PROFILE: &str =
    include_str!("./user/committee/GET_COMMITTEES_BY_PROFILE.sql");
pub const ADD_COMMITTEE: &str = include_str!("./user/committee/ADD_COMMITTEE.sql");
pub const UPDATE_COMMITTEE_ROLE: &str = include_str!("./user/committee/UPDATE_COMMITTEE_ROLE.sql");
pub const DELETE_COMMITTEE: &str = include_str!("./user/committee/DELETE_COMMITTEE.sql");

// Listing schema queries
// Category related queries
pub const GET_CATEGORIES: &str = include_str!("./listing/category/GET_CATEGORIES.sql");
pub const GET_CATEGORY_BY_ID: &str = include_str!("./listing/category/GET_CATEGORY_BY_ID.sql");
pub const ADD_CATEGORY: &str = include_str!("./listing/category/ADD_CATEGORY.sql");
pub const UPDATE_CATEGORY: &str = include_str!("./listing/category/UPDATE_CATEGORY.sql");

// Location related queries
pub const GET_LOCATIONS: &str = include_str!("./listing/location/GET_LOCATIONS.sql");
pub const GET_LOCATION_BY_ID: &str = include_str!("./listing/location/GET_LOCATION_BY_ID.sql");
pub const ADD_LOCATION: &str = include_str!("./listing/location/ADD_LOCATION.sql");
pub const UPDATE_LOCATION: &str = include_str!("./listing/location/UPDATE_LOCATION.sql");

// Item related queries
pub const GET_ITEMS: &str = include_str!("./listing/item/GET_ITEMS.sql");
pub const GET_ITEM_BY_ID: &str = include_str!("./listing/item/GET_ITEM_BY_ID.sql");

// Collection related queries
pub const GET_COLLECTIONS: &str = include_str!("./listing/collection/GET_COLLECTIONS.sql");
pub const GET_COLLECTION_BY_ID: &str =
    include_str!("./listing/collection/GET_COLLECTION_BY_ID.sql");
pub const GET_COLLECTIONS_BY_COMMUNITY: &str =
    include_str!("./listing/collection/GET_COLLECTIONS_BY_COMMUNITY.sql");
pub const ADD_COLLECTION: &str = include_str!("./listing/collection/ADD_COLLECTION.sql");
pub const UPDATE_COLLECTION: &str = include_str!("./listing/collection/UPDATE_COLLECTION.sql");

// Media related queries
pub const GET_MEDIA: &str = include_str!("./listing/media/GET_MEDIA.sql");
pub const GET_MEDIA_BY_ID: &str = include_str!("./listing/media/GET_MEDIA_BY_ID.sql");
pub const GET_MEDIA_BY_ITEM: &str = include_str!("./listing/media/GET_MEDIA_BY_ITEM.sql");
pub const ADD_MEDIA: &str = include_str!("./listing/media/ADD_MEDIA.sql");
pub const DELETE_MEDIA: &str = include_str!("./listing/media/DELETE_MEDIA.sql");

// Publish related queries
pub const GET_PUBLISHES: &str = include_str!("./listing/publish/GET_PUBLISHES.sql");
pub const GET_PUBLISH_BY_ITEM_AND_COLLECTION: &str =
    include_str!("./listing/publish/GET_PUBLISH_BY_ITEM_AND_COLLECTION.sql");
pub const GET_PUBLISHES_BY_COLLECTION: &str =
    include_str!("./listing/publish/GET_PUBLISHES_BY_COLLECTION.sql");
pub const GET_PUBLISHES_BY_ITEM: &str = include_str!("./listing/publish/GET_PUBLISHES_BY_ITEM.sql");
pub const ADD_PUBLISH: &str = include_str!("./listing/publish/ADD_PUBLISH.sql");
pub const UPDATE_PUBLISH: &str = include_str!("./listing/publish/UPDATE_PUBLISH.sql");
pub const DELETE_PUBLISH: &str = include_str!("./listing/publish/DELETE_PUBLISH.sql");

// Interaction schema queries
// Schedule related queries
pub const GET_SCHEDULES: &str = include_str!("./interaction/schedule/GET_SCHEDULES.sql");
pub const GET_SCHEDULE_BY_ID: &str = include_str!("./interaction/schedule/GET_SCHEDULE_BY_ID.sql");
pub const ADD_SCHEDULE: &str = include_str!("./interaction/schedule/ADD_SCHEDULE.sql");
pub const UPDATE_SCHEDULE: &str = include_str!("./interaction/schedule/UPDATE_SCHEDULE.sql");

// Pledge related queries
pub const GET_PLEDGES: &str = include_str!("./interaction/pledge/GET_PLEDGES.sql");
pub const GET_PLEDGE_BY_ID: &str = include_str!("./interaction/pledge/GET_PLEDGE_BY_ID.sql");
pub const GET_PLEDGES_BY_ITEM: &str = include_str!("./interaction/pledge/GET_PLEDGES_BY_ITEM.sql");
pub const GET_PLEDGES_BY_PROFILE: &str =
    include_str!("./interaction/pledge/GET_PLEDGES_BY_PROFILE.sql");
pub const ADD_PLEDGE: &str = include_str!("./interaction/pledge/ADD_PLEDGE.sql");

// Transaction related queries
pub const GET_TRANSACTIONS: &str = include_str!("./interaction/transaction/GET_TRANSACTIONS.sql");
pub const GET_TRANSACTION_BY_ID: &str =
    include_str!("./interaction/transaction/GET_TRANSACTION_BY_ID.sql");
pub const GET_TRANSACTIONS_BY_PLEDGE: &str =
    include_str!("./interaction/transaction/GET_TRANSACTIONS_BY_PLEDGE.sql");
pub const ADD_TRANSACTION: &str = include_str!("./interaction/transaction/ADD_TRANSACTION.sql");
pub const UPDATE_TRANSACTION: &str =
    include_str!("./interaction/transaction/UPDATE_TRANSACTION.sql");

// Message related queries
pub const GET_MESSAGES: &str = include_str!("./interaction/message/GET_MESSAGES.sql");
pub const GET_MESSAGE_BY_ID: &str = include_str!("./interaction/message/GET_MESSAGE_BY_ID.sql");
pub const GET_MESSAGES_BY_TRANSACTION: &str =
    include_str!("./interaction/message/GET_MESSAGES_BY_TRANSACTION.sql");
pub const ADD_MESSAGE: &str = include_str!("./interaction/message/ADD_MESSAGE.sql");

// Review related queries
pub const GET_REVIEWS: &str = include_str!("./interaction/review/GET_REVIEWS.sql");
pub const GET_REVIEW_BY_TRANSACTION_AND_SUBJECT: &str =
    include_str!("./interaction/review/GET_REVIEW_BY_TRANSACTION_AND_SUBJECT.sql");

// Schedule opportunity related queries
pub const GET_SCHEDULE_OPPORTUNITIES: &str =
    include_str!("./interaction/schedule_opportunity/GET_SCHEDULE_OPPORTUNITIES.sql");
pub const GET_SCHEDULE_OPPORTUNITY_BY_ID: &str =
    include_str!("./interaction/schedule_opportunity/GET_SCHEDULE_OPPORTUNITY_BY_ID.sql");
pub const ADD_SCHEDULE_OPPORTUNITY: &str =
    include_str!("./interaction/schedule_opportunity/ADD_SCHEDULE_OPPORTUNITY.sql");
pub const UPDATE_SCHEDULE_OPPORTUNITY: &str =
    include_str!("./interaction/schedule_opportunity/UPDATE_SCHEDULE_OPPORTUNITY.sql");
