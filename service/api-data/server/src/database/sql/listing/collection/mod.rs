use std::fs;
use std::path::Path;

pub const GET_COLLECTIONS: &str = include_str!("GET_COLLECTIONS.sql");
pub const GET_COLLECTION_BY_ID: &str = include_str!("GET_COLLECTION_BY_ID.sql");
pub const GET_COLLECTIONS_BY_PROFILE: &str = include_str!("GET_COLLECTIONS_BY_PROFILE.sql");
pub const ADD_COLLECTION: &str = include_str!("ADD_COLLECTION.sql");
pub const UPDATE_COLLECTION: &str = include_str!("UPDATE_COLLECTION.sql");
pub const DELETE_COLLECTION: &str = include_str!("DELETE_COLLECTION.sql");
