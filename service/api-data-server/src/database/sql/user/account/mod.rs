use std::fs;
use std::path::Path;

// SQL queries for account operations
pub const LIST_ACCOUNTS: &str = include_str!("LIST_ACCOUNTS.sql");
pub const FIND_ACCOUNT: &str = include_str!("FIND_ACCOUNT.sql");
pub const CREATE_ACCOUNT: &str = include_str!("CREATE_ACCOUNT.sql");
pub const UPDATE_ACCOUNT: &str = include_str!("UPDATE_ACCOUNT.sql");
pub const DELETE_ACCOUNT: &str = include_str!("DELETE_ACCOUNT.sql");
