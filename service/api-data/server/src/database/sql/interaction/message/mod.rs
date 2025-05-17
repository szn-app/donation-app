use std::fs;
use std::path::Path;

pub const GET_MESSAGES: &str = include_str!("GET_MESSAGES.sql");
pub const GET_MESSAGE_BY_ID: &str = include_str!("GET_MESSAGE_BY_ID.sql");
pub const GET_MESSAGES_BY_TRANSACTION: &str = include_str!("GET_MESSAGES_BY_TRANSACTION.sql");
pub const ADD_MESSAGE: &str = include_str!("ADD_MESSAGE.sql");
