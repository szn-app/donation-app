use std::fs;
use std::path::Path;

pub const GET_REVIEWS: &str = include_str!("GET_REVIEWS.sql");
pub const GET_REVIEW_BY_ID: &str = include_str!("GET_REVIEW_BY_ID.sql");
pub const ADD_REVIEW: &str = include_str!("ADD_REVIEW.sql");
pub const GET_REVIEW_BY_TRANSACTION_AND_SUBJECT: &str =
    include_str!("GET_REVIEW_BY_TRANSACTION_AND_SUBJECT.sql");
