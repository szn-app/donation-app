use std::fs;
use std::path::Path;

pub const LIST_MESSAGES: &str = include_str!("LIST_MESSAGES.sql");
pub const FIND_MESSAGE: &str = include_str!("FIND_MESSAGE.sql");
pub const FIND_MESSAGES_BY_TRANSACTION: &str = include_str!("FIND_MESSAGES_BY_TRANSACTION.sql");
pub const CREATE_MESSAGE: &str = include_str!("CREATE_MESSAGE.sql");
