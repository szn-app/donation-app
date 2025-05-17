use std::fs;
use std::path::Path;

pub const GET_PUBLISHES: &str = include_str!("GET_PUBLISHES.sql");
pub const GET_PUBLISH_BY_ID: &str = include_str!("GET_PUBLISH_BY_ID.sql");
pub const GET_PUBLISHES_BY_COLLECTION: &str = include_str!("GET_PUBLISHES_BY_COLLECTION.sql");
pub const GET_PUBLISHES_BY_ITEM: &str = include_str!("GET_PUBLISHES_BY_ITEM.sql");
pub const GET_PUBLISH_BY_ITEM_AND_COLLECTION: &str =
    include_str!("GET_PUBLISH_BY_ITEM_AND_COLLECTION.sql");
pub const ADD_PUBLISH: &str = include_str!("ADD_PUBLISH.sql");
pub const UPDATE_PUBLISH: &str = include_str!("UPDATE_PUBLISH.sql");
pub const DELETE_PUBLISH: &str = include_str!("DELETE_PUBLISH.sql");
