use std::fs;
use std::path::Path;

pub const LIST_COLLECTIONS: &str = include_str!("LIST_COLLECTIONS.sql");
pub const FIND_COLLECTION: &str = include_str!("FIND_COLLECTION.sql");
pub const FIND_COLLECTIONS_BY_PROFILE: &str = include_str!("FIND_COLLECTIONS_BY_PROFILE.sql");
pub const CREATE_COLLECTION: &str = include_str!("CREATE_COLLECTION.sql");
pub const UPDATE_COLLECTION: &str = include_str!("UPDATE_COLLECTION.sql");
pub const DELETE_COLLECTION: &str = include_str!("DELETE_COLLECTION.sql");
