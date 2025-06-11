use std::fs;
use std::path::Path;

pub const CREATE_PUBLISH: &str = include_str!("CREATE_PUBLISH.sql");
pub const DELETE_PUBLISH: &str = include_str!("DELETE_PUBLISH.sql");
pub const FIND_PUBLISH: &str = include_str!("FIND_PUBLISH.sql");
pub const FIND_PUBLISH_BY_ITEM: &str = include_str!("FIND_PUBLISH_BY_ITEM.sql");
pub const FIND_PUBLISH_BY_COLLECTION: &str = include_str!("FIND_PUBLISH_BY_COLLECTION.sql");
pub const FIND_PUBLISH_BY_ITEM_AND_COLLECTION: &str = include_str!("FIND_PUBLISH_BY_ITEM_AND_COLLECTION.sql");
pub const FIND_PUBLISHES_BY_COLLECTION: &str = include_str!("FIND_PUBLISHES_BY_COLLECTION.sql");
pub const FIND_PUBLISHES_BY_ITEM: &str = include_str!("FIND_PUBLISHES_BY_ITEM.sql");
pub const LIST_PUBLISHES: &str = include_str!("LIST_PUBLISHES.sql");
pub const UPDATE_PUBLISH: &str = include_str!("UPDATE_PUBLISH.sql");
