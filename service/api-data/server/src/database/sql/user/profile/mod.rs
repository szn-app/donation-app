use std::fs;
use std::path::Path;

pub const LIST_PROFILES: &str = include_str!("LIST_PROFILES.sql");
pub const FIND_PROFILE: &str = include_str!("FIND_PROFILE.sql");
pub const FIND_PROFILE_BY_ACCOUNT: &str = include_str!("FIND_PROFILE_BY_ACCOUNT.sql");
pub const CREATE_PROFILE: &str = include_str!("CREATE_PROFILE.sql");
pub const FIND_PROFILES_BY_OWNER: &str = include_str!("FIND_PROFILES_BY_OWNER.sql");
pub const UPDATE_PROFILE: &str = include_str!("UPDATE_PROFILE.sql");
pub const DELETE_PROFILE: &str = include_str!("DELETE_PROFILE.sql");
