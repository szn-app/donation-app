use std::fs;
use std::path::Path;

pub const LIST_PLEDGES: &str = include_str!("LIST_PLEDGES.sql");
pub const FIND_PLEDGE: &str = include_str!("FIND_PLEDGE.sql");
pub const FIND_PLEDGES_BY_ITEM: &str = include_str!("FIND_PLEDGES_BY_ITEM.sql");
pub const FIND_PLEDGES_BY_PROFILE: &str = include_str!("FIND_PLEDGES_BY_PROFILE.sql");
pub const CREATE_PLEDGE: &str = include_str!("CREATE_PLEDGE.sql");
pub const UPDATE_PLEDGE: &str = include_str!("UPDATE_PLEDGE.sql");
