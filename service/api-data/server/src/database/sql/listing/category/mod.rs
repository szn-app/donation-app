use std::fs;
use std::path::Path;

pub const LIST_CATEGORIES: &str = include_str!("LIST_CATEGORIES.sql");
pub const FIND_CATEGORY: &str = include_str!("FIND_CATEGORY.sql");
pub const FIND_CATEGORIES_BY_PARENT: &str = include_str!("FIND_CATEGORIES_BY_PARENT.sql");
pub const CREATE_CATEGORY: &str = include_str!("CREATE_CATEGORY.sql");
pub const UPDATE_CATEGORY: &str = include_str!("UPDATE_CATEGORY.sql");
pub const DELETE_CATEGORY: &str = include_str!("DELETE_CATEGORY.sql");
