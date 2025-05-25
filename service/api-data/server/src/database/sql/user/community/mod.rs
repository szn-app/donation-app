use std::fs;
use std::path::Path;

pub const GET_COMMUNITIES: &str = include_str!("GET_COMMUNITIES.sql");
pub const GET_COMMUNITY_BY_ID: &str = include_str!("GET_COMMUNITY_BY_ID.sql");
pub const GET_COMMUNITIES_BY_PROFILE: &str = include_str!("GET_COMMUNITIES_BY_PROFILE.sql");
pub const CREATE_COMMUNITY: &str = include_str!("CREATE_COMMUNITY.sql");
pub const GET_COMMUNITIES_BY_OWNER: &str = include_str!("GET_COMMUNITIES_BY_OWNER.sql");
pub const UPDATE_COMMUNITY: &str = include_str!("UPDATE_COMMUNITY.sql");
pub const DELETE_COMMUNITY: &str = include_str!("DELETE_COMMUNITY.sql");
