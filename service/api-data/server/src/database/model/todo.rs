/*

TODO:

Handling Spatial Types (GEOGRAPHY, VECTOR):
   The `geom` column in `listing.Location` uses the `extension.geography` type (specifically `Point, 4326`). The `vector` type is also an extension.
   `tokio-postgres` does not have built-in support for these. You will need to:
   a) Choose a Rust crate for representing these spatial types (e.g., `geo-types` for Point).
   b) Manually implement the `tokio_postgres::types::FromSql` and `tokio_postgres::types::ToSql` traits for the chosen Rust spatial type, handling the conversion to/from the database's binary representation of these types. This can be complex. Alternatively, you might fetch these as `String` or `Vec<u8>` and parse them using a spatial library if direct binary mapping is too difficult.
   // You will also need to implement FromSql and ToSql for these types if using tokio-postgres
   // You might need a crate for spatial types, e.g., `geo-types`
   // use geo_types::Point; // Example placeholder

*/

// --- ENUM Types (Schema: public) ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CollectionType {
    Featured,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CollectionVisibility {
    Public,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
// --- ENUM Types (Schema: public) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CollectionType {
    Featured,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CollectionVisibility {
    Public,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CommitteeRole {
    Organizer,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CommunityType {
    Solo,
    Organized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemCondition {
    BrandNew,
    PreOwnedBarelyUsed,
    PreOwnedUsable,
    PreOwnedDamaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemIntentAction {
    Request,
    Offer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemStatus {
    Draft,
    Active,
    Disabled,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemType {
    InKind,
    Inquiry,
    Monetary,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum MessageType {
    Text,
    ScheduleOpportunity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum PledgeIntentAction {
    Give,
    Receive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum PledgeStatus {
    Pending,
    Approved,
    Declined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ProfileType {
    Individual,
    Organization,
    Company,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum TransactionStatus {
    InProgress,
    Completed,
    Cancelled,
}

// --- Tables (Organized by Schema) ---

// Schema: interaction
pub mod interaction {
    use super::{
        MessageType, OffsetDateTime, PledgeIntentAction, PledgeStatus, TransactionStatus, Uuid,
    };
    // use super::Point; // Example placeholder for geo spatial types

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub id: i64,                            // bigint
        pub id_sender: Option<i64>,             // bigint, NULL
        pub id_transaction: Option<i64>,        // bigint, NULL
        pub type_: MessageType, // message_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub content: String,    // text, NOT NULL
        pub sent_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pledge {
        pub id: i64,                            // bigint
        pub id_profile: Option<i64>,            // bigint, NULL
        pub id_item: Option<i64>,               // bigint, NULL
        pub intent_action: PledgeIntentAction,  // pledge_intent_action (ENUM), NOT NULL
        pub message: Option<String>,            // text, NULL
        pub status: PledgeStatus,               // pledge_status (ENUM), NOT NULL
        pub pledged_at: OffsetDateTime,         // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub updated_by: Option<Uuid>,           // uuid, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_transaction, id_subject_profile)
    pub struct Review {
        pub id_transaction: i64,        // bigint, NOT NULL
        pub id_subject_profile: i64,    // bigint, NOT NULL
        pub reviewer: Option<i64>,      // bigint, NULL
        pub comment: Option<String>,    // text, NULL
        pub score: i16,                 // smallint, NOT NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Schedule {
        pub id: i64,                       // bigint
        pub scheduled_for: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table seems to be linking message and schedule_opportunity, id_message references message.id
    pub struct ScheduleOpportunity {
        // Assuming `id` here corresponds to the message id
        pub id: Option<i64>, // bigint, NULL - Note: FK to message(id), but nullable in this dump
        pub window_start: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub window_end: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Transaction {
        pub id: i64,                            // bigint
        pub id_pledge: Option<i64>,             // bigint, NULL
        pub status: TransactionStatus,          // transaction_status (ENUM), NOT NULL
        pub id_schedule: Option<i64>,           // bigint, NULL
        pub id_location: Option<i64>,           // bigint, NULL
        pub created_at: OffsetDateTime,         // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }
}

// Schema: listing
pub mod listing {
    use super::{
        CollectionType, CollectionVisibility, ItemCondition, ItemIntentAction, ItemStatus,
        ItemType, MediaType, OffsetDateTime, Uuid,
    };
    // use super::Point; // Example placeholder for geo spatial types

    #[derive(Debug, Clone, PartialEq)]
    pub struct Category {
        pub id: i64,                      // bigint
        pub title: String,                // varchar(150), NOT NULL
        pub description: Option<String>,  // text, NULL
        pub category_parent: Option<i64>, // bigint, NULL (FK to self)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Collection {
        pub id: i64,                            // bigint
        pub id_community: Option<i64>,          // bigint, NULL
        pub title: Option<String>,              // varchar(150), NULL
        pub visibility: CollectionVisibility,   // collection_visibility (ENUM), NOT NULL
        pub type_: Option<CollectionType>, // collection_type (ENUM), NULL - Note: Renamed to avoid Rust keyword 'type'
        pub position: i32,                 // integer, NOT NULL
        pub created_at: OffsetDateTime,    // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Item {
        pub id: i64,                            // bigint
        pub type_: ItemType, // item_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub intent_action: ItemIntentAction, // item_intent_action (ENUM), NOT NULL
        pub status: ItemStatus, // item_status (ENUM), NOT NULL
        pub title: Option<String>, // varchar(150), NULL
        pub description: Option<String>, // text, NULL
        pub category: Option<i64>, // bigint, NULL (FK to category)
        pub condition: ItemCondition, // item_condition (ENUM), NOT NULL
        pub location: Option<i64>, // bigint, NULL (FK to location)
        pub views_count: i64, // bigint, NOT NULL
        pub is_reported: bool, // boolean, NOT NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Option<Uuid>, // uuid, NULL (FK to user.account)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Location {
        pub id: i64,                       // bigint
        pub address_line1: String,         // varchar(64), NOT NULL
        pub address_line2: Option<String>, // varchar(64), NULL
        pub city: String,                  // varchar(50), NOT NULL
        pub state: Option<String>,         // varchar(50), NULL
        pub district: Option<i64>,         // bigint, NULL - Note: No FK defined for district?
        pub country: String,               // varchar(50), NOT NULL
        // Example placeholder for GEOGRAPHY(Point, 4326)
        // You will need to implement tokio_postgres::types::FromSql and ToSql for your spatial type
        // pub geom: Option<Point>, // geography(Point, 4326), NULL
        pub geom: Option<String>, // Placeholder as String, requires custom FromSql/ToSql
        pub entrance_note: Option<String>, // text, NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Media {
        pub id: i64,                    // bigint
        pub id_item: Option<i64>,       // bigint, NULL
        pub caption: Option<String>,    // varchar(150), NULL
        pub url: String,                // varchar(2048), NOT NULL
        pub type_: MediaType, // media_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_item, id_collection)
    pub struct Publish {
        pub id_item: i64,              // bigint, NOT NULL (FK to item)
        pub id_collection: i64,        // bigint, NOT NULL (FK to collection)
        pub note: Option<String>,      // text, NULL
        pub position: i32,             // integer, NOT NULL
        pub added_by: Option<Uuid>,    // uuid, NULL (FK to user.account)
        pub posted_on: OffsetDateTime, // timestamp with time zone, NOT NULL
    }
}

// Schema: user
pub mod user {
    use super::{CommitteeRole, CommunityType, OffsetDateTime, ProfileType, Uuid};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Account {
        pub id: Uuid,                   // uuid, NOT NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_profile, id_community)
    pub struct Committee {
        pub id_profile: i64,            // bigint, NOT NULL (FK to user.profile)
        pub id_community: i64,          // bigint, NOT NULL (FK to user.community)
        pub member_role: CommitteeRole, // committee_role (ENUM), NOT NULL
        pub joined_at: OffsetDateTime,  // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Community {
        pub id: i64,                            // bigint
        pub title: Option<String>,              // varchar(150), NULL
        pub description: Option<String>,        // text, NULL
        pub type_: CommunityType, // community_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub owner: Uuid,          // uuid, NOT NULL (FK to user.account)
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Uuid,     // uuid, NOT NULL (FK to user.account)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Profile {
        pub id: i64,                            // bigint
        pub name: Option<String>,               // varchar(100), NULL
        pub description: Option<String>,        // text, NULL
        pub type_: Option<ProfileType>, // profile_type (ENUM), NULL - Note: Renamed to avoid Rust keyword 'type'
        pub owner: Uuid,                // uuid, NOT NULL (FK to user.account)
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Uuid,           // uuid, NOT NULL (FK to user.account)
    }
}

// Schema: public (tables not in other schemas)
pub mod public {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Test {
        pub i: Option<i32>, // integer, NULL
    }
}

pub enum CommitteeRole {
    Organizer,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum CommunityType {
    Solo,
    Organized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemCondition {
    BrandNew,
    PreOwnedBarelyUsed,
    PreOwnedUsable,
    PreOwnedDamaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemIntentAction {
    Request,
    Offer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemStatus {
    Draft,
    Active,
    Disabled,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ItemType {
    InKind,
    Inquiry,
    Monetary,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum MessageType {
    Text,
    ScheduleOpportunity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum PledgeIntentAction {
    Give,
    Receive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum PledgeStatus {
    Pending,
    Approved,
    Declined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum ProfileType {
    Individual,
    Organization,
    Company,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// You will need to implement tokio_postgres::types::FromSql and ToSql for this enum
pub enum TransactionStatus {
    InProgress,
    Completed,
    Cancelled,
}

// --- Tables (Organized by Schema) ---

// Schema: interaction
pub mod interaction {
    use super::{
        MessageType, OffsetDateTime, PledgeIntentAction, PledgeStatus, TransactionStatus, Uuid,
    };
    // use super::Point; // Example placeholder for geo spatial types

    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub id: i64,                            // bigint
        pub id_sender: Option<i64>,             // bigint, NULL
        pub id_transaction: Option<i64>,        // bigint, NULL
        pub type_: MessageType, // message_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub content: String,    // text, NOT NULL
        pub sent_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pledge {
        pub id: i64,                            // bigint
        pub id_profile: Option<i64>,            // bigint, NULL
        pub id_item: Option<i64>,               // bigint, NULL
        pub intent_action: PledgeIntentAction,  // pledge_intent_action (ENUM), NOT NULL
        pub message: Option<String>,            // text, NULL
        pub status: PledgeStatus,               // pledge_status (ENUM), NOT NULL
        pub pledged_at: OffsetDateTime,         // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub updated_by: Option<Uuid>,           // uuid, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_transaction, id_subject_profile)
    pub struct Review {
        pub id_transaction: i64,        // bigint, NOT NULL
        pub id_subject_profile: i64,    // bigint, NOT NULL
        pub reviewer: Option<i64>,      // bigint, NULL
        pub comment: Option<String>,    // text, NULL
        pub score: i16,                 // smallint, NOT NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Schedule {
        pub id: i64,                       // bigint
        pub scheduled_for: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table seems to be linking message and schedule_opportunity, id_message references message.id
    pub struct ScheduleOpportunity {
        // Assuming `id` here corresponds to the message id
        pub id: Option<i64>, // bigint, NULL - Note: FK to message(id), but nullable in this dump
        pub window_start: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub window_end: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Transaction {
        pub id: i64,                            // bigint
        pub id_pledge: Option<i64>,             // bigint, NULL
        pub status: TransactionStatus,          // transaction_status (ENUM), NOT NULL
        pub id_schedule: Option<i64>,           // bigint, NULL
        pub id_location: Option<i64>,           // bigint, NULL
        pub created_at: OffsetDateTime,         // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }
}

// Schema: listing
pub mod listing {
    use super::{
        CollectionType, CollectionVisibility, ItemCondition, ItemIntentAction, ItemStatus,
        ItemType, MediaType, OffsetDateTime, Uuid,
    };
    // use super::Point; // Example placeholder for geo spatial types

    #[derive(Debug, Clone, PartialEq)]
    pub struct Category {
        pub id: i64,                      // bigint
        pub title: String,                // varchar(150), NOT NULL
        pub description: Option<String>,  // text, NULL
        pub category_parent: Option<i64>, // bigint, NULL (FK to self)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Collection {
        pub id: i64,                            // bigint
        pub id_community: Option<i64>,          // bigint, NULL
        pub title: Option<String>,              // varchar(150), NULL
        pub visibility: CollectionVisibility,   // collection_visibility (ENUM), NOT NULL
        pub type_: Option<CollectionType>, // collection_type (ENUM), NULL - Note: Renamed to avoid Rust keyword 'type'
        pub position: i32,                 // integer, NOT NULL
        pub created_at: OffsetDateTime,    // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Item {
        pub id: i64,                            // bigint
        pub type_: ItemType, // item_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub intent_action: ItemIntentAction, // item_intent_action (ENUM), NOT NULL
        pub status: ItemStatus, // item_status (ENUM), NOT NULL
        pub title: Option<String>, // varchar(150), NULL
        pub description: Option<String>, // text, NULL
        pub category: Option<i64>, // bigint, NULL (FK to category)
        pub condition: ItemCondition, // item_condition (ENUM), NOT NULL
        pub location: Option<i64>, // bigint, NULL (FK to location)
        pub views_count: i64, // bigint, NOT NULL
        pub is_reported: bool, // boolean, NOT NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Option<Uuid>, // uuid, NULL (FK to user.account)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Location {
        pub id: i64,                       // bigint
        pub address_line1: String,         // varchar(64), NOT NULL
        pub address_line2: Option<String>, // varchar(64), NULL
        pub city: String,                  // varchar(50), NOT NULL
        pub state: Option<String>,         // varchar(50), NULL
        pub district: Option<i64>,         // bigint, NULL - Note: No FK defined for district?
        pub country: String,               // varchar(50), NOT NULL
        // Example placeholder for GEOGRAPHY(Point, 4326)
        // You will need to implement tokio_postgres::types::FromSql and ToSql for your spatial type
        // pub geom: Option<Point>, // geography(Point, 4326), NULL
        pub geom: Option<String>, // Placeholder as String, requires custom FromSql/ToSql
        pub entrance_note: Option<String>, // text, NULL
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Media {
        pub id: i64,                    // bigint
        pub id_item: Option<i64>,       // bigint, NULL
        pub caption: Option<String>,    // varchar(150), NULL
        pub url: String,                // varchar(2048), NOT NULL
        pub type_: MediaType, // media_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_item, id_collection)
    pub struct Publish {
        pub id_item: i64,              // bigint, NOT NULL (FK to item)
        pub id_collection: i64,        // bigint, NOT NULL (FK to collection)
        pub note: Option<String>,      // text, NULL
        pub position: i32,             // integer, NOT NULL
        pub added_by: Option<Uuid>,    // uuid, NULL (FK to user.account)
        pub posted_on: OffsetDateTime, // timestamp with time zone, NOT NULL
    }
}

// Schema: user
pub mod user {
    use super::{CommitteeRole, CommunityType, OffsetDateTime, ProfileType, Uuid};

    #[derive(Debug, Clone, PartialEq)]
    // This table has a composite primary key (id_profile, id_community)
    pub struct Committee {
        pub id_profile: i64,            // bigint, NOT NULL (FK to user.profile)
        pub id_community: i64,          // bigint, NOT NULL (FK to user.community)
        pub member_role: CommitteeRole, // committee_role (ENUM), NOT NULL
        pub joined_at: OffsetDateTime,  // timestamp with time zone, NOT NULL
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Community {
        pub id: i64,                            // bigint
        pub title: Option<String>,              // varchar(150), NULL
        pub description: Option<String>,        // text, NULL
        pub type_: CommunityType, // community_type (ENUM), NOT NULL - Note: Renamed to avoid Rust keyword 'type'
        pub owner: Uuid,          // uuid, NOT NULL (FK to user.account)
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Uuid,     // uuid, NOT NULL (FK to user.account)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Profile {
        pub id: i64,                            // bigint
        pub name: Option<String>,               // varchar(100), NULL
        pub description: Option<String>,        // text, NULL
        pub type_: Option<ProfileType>, // profile_type (ENUM), NULL - Note: Renamed to avoid Rust keyword 'type'
        pub owner: Uuid,                // uuid, NOT NULL (FK to user.account)
        pub created_at: OffsetDateTime, // timestamp with time zone, NOT NULL
        pub updated_at: Option<OffsetDateTime>, // timestamp with time zone, NULL
        pub created_by: Uuid,           // uuid, NOT NULL (FK to user.account)
    }
}

// Schema: public (tables not in other schemas)
pub mod public {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Test {
        pub i: Option<i32>, // integer, NULL
    }
}
