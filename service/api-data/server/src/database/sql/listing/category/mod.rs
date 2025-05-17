use std::fs;
use std::path::Path;

pub const GET_CATEGORIES: &str = include_str!("GET_CATEGORIES.sql");
pub const GET_CATEGORY_BY_ID: &str = include_str!("GET_CATEGORY_BY_ID.sql");
pub const GET_CATEGORIES_BY_PARENT: &str = include_str!("GET_CATEGORIES_BY_PARENT.sql");
pub const ADD_CATEGORY: &str = include_str!("ADD_CATEGORY.sql");
pub const UPDATE_CATEGORY: &str = include_str!("UPDATE_CATEGORY.sql");
pub const DELETE_CATEGORY: &str = include_str!("DELETE_CATEGORY.sql");
