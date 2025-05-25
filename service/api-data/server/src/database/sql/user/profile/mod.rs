use std::fs;
use std::path::Path;

pub const GET_PROFILES: &str = include_str!("GET_PROFILES.sql");
pub const GET_PROFILE_BY_ID: &str = include_str!("GET_PROFILE_BY_ID.sql");
pub const GET_PROFILE_BY_ACCOUNT: &str = include_str!("GET_PROFILE_BY_ACCOUNT.sql");
pub const CREATE_PROFILE: &str = include_str!("CREATE_PROFILE.sql");
pub const GET_PROFILES_BY_OWNER: &str = include_str!("GET_PROFILES_BY_OWNER.sql");
pub const UPDATE_PROFILE: &str = include_str!("UPDATE_PROFILE.sql");
pub const DELETE_PROFILE: &str = include_str!("DELETE_PROFILE.sql");
