use std::fs;
use std::path::Path;

pub const LIST_ITEMS: &str = include_str!("LIST_ITEMS.sql");
pub const FIND_ITEM: &str = include_str!("FIND_ITEM.sql");
pub const FIND_ITEMS_BY_PROFILE: &str = include_str!("FIND_ITEMS_BY_PROFILE.sql");
pub const FIND_ITEMS_BY_CATEGORY: &str = include_str!("FIND_ITEMS_BY_CATEGORY.sql");
pub const CREATE_ITEM: &str = include_str!("CREATE_ITEM.sql");
pub const UPDATE_ITEM: &str = include_str!("UPDATE_ITEM.sql");
pub const DELETE_ITEM: &str = include_str!("DELETE_ITEM.sql");
pub const INCREMENT_VIEWS: &str = include_str!("INCREMENT_VIEWS.sql");
pub const REPORT_ITEM: &str = include_str!("REPORT_ITEM.sql");
