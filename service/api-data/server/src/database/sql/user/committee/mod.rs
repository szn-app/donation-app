use std::fs;
use std::path::Path;

pub const GET_COMMITTEES: &str = include_str!("GET_COMMITTEES.sql");
pub const GET_COMMITTEE_BY_ID: &str = include_str!("GET_COMMITTEE_BY_ID.sql");
pub const GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY: &str = include_str!("GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY.sql");
pub const GET_COMMITTEES_BY_COMMUNITY: &str = include_str!("GET_COMMITTEES_BY_COMMUNITY.sql");
pub const GET_COMMITTEES_BY_PROFILE: &str = include_str!("GET_COMMITTEES_BY_PROFILE.sql");
pub const CREATE_COMMITTEE: &str = include_str!("CREATE_COMMITTEE.sql");
pub const UPDATE_COMMITTEE_ROLE: &str = include_str!("UPDATE_COMMITTEE_ROLE.sql");
pub const DELETE_COMMITTEE: &str = include_str!("DELETE_COMMITTEE.sql");
