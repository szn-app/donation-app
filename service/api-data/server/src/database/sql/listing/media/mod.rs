use std::fs;
use std::path::Path;

pub const GET_MEDIA: &str = include_str!("GET_MEDIA.sql");
pub const GET_MEDIA_BY_ID: &str = include_str!("GET_MEDIA_BY_ID.sql");
pub const GET_MEDIA_BY_ITEM: &str = include_str!("GET_MEDIA_BY_ITEM.sql");
pub const ADD_MEDIA: &str = include_str!("ADD_MEDIA.sql");
pub const UPDATE_MEDIA: &str = include_str!("UPDATE_MEDIA.sql");
pub const DELETE_MEDIA: &str = include_str!("DELETE_MEDIA.sql");
