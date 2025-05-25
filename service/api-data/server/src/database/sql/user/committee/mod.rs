use std::fs;
use std::path::Path;

pub const LIST_COMMITTEES: &str = include_str!("LIST_COMMITTEES.sql");
pub const FIND_COMMITTEE: &str = include_str!("FIND_COMMITTEE.sql");
pub const FIND_COMMITTEE_BY_PROFILE_AND_COMMUNITY: &str = include_str!("FIND_COMMITTEE_BY_PROFILE_AND_COMMUNITY.sql");
pub const FIND_COMMITTEES_BY_COMMUNITY: &str = include_str!("FIND_COMMITTEES_BY_COMMUNITY.sql");
pub const FIND_COMMITTEES_BY_PROFILE: &str = include_str!("FIND_COMMITTEES_BY_PROFILE.sql");
pub const CREATE_COMMITTEE: &str = include_str!("CREATE_COMMITTEE.sql");
pub const UPDATE_COMMITTEE_ROLE: &str = include_str!("UPDATE_COMMITTEE_ROLE.sql");
pub const DELETE_COMMITTEE: &str = include_str!("DELETE_COMMITTEE.sql");
