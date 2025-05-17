use std::fs;
use std::path::Path;

pub const GET_TRANSACTIONS: &str = include_str!("GET_TRANSACTIONS.sql");
pub const GET_TRANSACTION_BY_ID: &str = include_str!("GET_TRANSACTION_BY_ID.sql");
pub const GET_TRANSACTIONS_BY_PLEDGE: &str = include_str!("GET_TRANSACTIONS_BY_PLEDGE.sql");
pub const ADD_TRANSACTION: &str = include_str!("ADD_TRANSACTION.sql");
pub const UPDATE_TRANSACTION: &str = include_str!("UPDATE_TRANSACTION.sql");
