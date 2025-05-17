use std::fs;
use std::path::Path;

pub const GET_ITEMS: &str = include_str!("GET_ITEMS.sql");
pub const GET_ITEM_BY_ID: &str = include_str!("GET_ITEM_BY_ID.sql");
pub const GET_ITEMS_BY_PROFILE: &str = include_str!("GET_ITEMS_BY_PROFILE.sql");
pub const GET_ITEMS_BY_CATEGORY: &str = include_str!("GET_ITEMS_BY_CATEGORY.sql");
pub const ADD_ITEM: &str = include_str!("ADD_ITEM.sql");
pub const UPDATE_ITEM: &str = include_str!("UPDATE_ITEM.sql");
pub const DELETE_ITEM: &str = include_str!("DELETE_ITEM.sql");
