use std::fs;
use std::path::Path;

pub const LIST_TRANSACTIONS: &str = include_str!("LIST_TRANSACTIONS.sql");
pub const FIND_TRANSACTION: &str = include_str!("FIND_TRANSACTION.sql");
pub const FIND_TRANSACTIONS_BY_PLEDGE: &str = include_str!("FIND_TRANSACTIONS_BY_PLEDGE.sql");
pub const CREATE_TRANSACTION: &str = include_str!("CREATE_TRANSACTION.sql");
pub const UPDATE_TRANSACTION: &str = include_str!("UPDATE_TRANSACTION.sql");
