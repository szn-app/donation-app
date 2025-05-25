use std::fs;
use std::path::Path;

pub const LIST_COMMUNITIES: &str = include_str!("LIST_COMMUNITIES.sql");
pub const FIND_COMMUNITY: &str = include_str!("FIND_COMMUNITY.sql");
pub const FIND_COMMUNITIES_BY_PROFILE: &str = include_str!("FIND_COMMUNITIES_BY_PROFILE.sql");
pub const CREATE_COMMUNITY: &str = include_str!("CREATE_COMMUNITY.sql");
pub const FIND_COMMUNITIES_BY_OWNER: &str = include_str!("FIND_COMMUNITIES_BY_OWNER.sql");
pub const UPDATE_COMMUNITY: &str = include_str!("UPDATE_COMMUNITY.sql");
pub const DELETE_COMMUNITY: &str = include_str!("DELETE_COMMUNITY.sql");
