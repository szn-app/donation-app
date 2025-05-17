use std::fs;
use std::path::Path;

pub const GET_ACCOUNTS: &str = include_str!("GET_ACCOUNTS.sql");
pub const GET_ACCOUNT_BY_ID: &str = include_str!("GET_ACCOUNT_BY_ID.sql");
pub const GET_ACCOUNT_BY_EMAIL: &str = include_str!("GET_ACCOUNT_BY_EMAIL.sql");
pub const ADD_ACCOUNT: &str = include_str!("ADD_ACCOUNT.sql");
pub const UPDATE_ACCOUNT: &str = include_str!("UPDATE_ACCOUNT.sql");
pub const DELETE_ACCOUNT: &str = include_str!("DELETE_ACCOUNT.sql");
