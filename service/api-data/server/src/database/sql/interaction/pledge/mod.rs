use std::fs;
use std::path::Path;

pub const GET_PLEDGES: &str = include_str!("GET_PLEDGES.sql");
pub const GET_PLEDGE_BY_ID: &str = include_str!("GET_PLEDGE_BY_ID.sql");
pub const GET_PLEDGES_BY_ITEM: &str = include_str!("GET_PLEDGES_BY_ITEM.sql");
pub const GET_PLEDGES_BY_PROFILE: &str = include_str!("GET_PLEDGES_BY_PROFILE.sql");
pub const ADD_PLEDGE: &str = include_str!("ADD_PLEDGE.sql");
pub const UPDATE_PLEDGE: &str = include_str!("UPDATE_PLEDGE.sql");
