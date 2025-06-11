use std::fs;
use std::path::Path;

pub const LIST_REVIEWS: &str = include_str!("LIST_REVIEWS.sql");
pub const FIND_REVIEW: &str = include_str!("FIND_REVIEW.sql");
pub const CREATE_REVIEW: &str = include_str!("CREATE_REVIEW.sql");
pub const FIND_REVIEW_BY_TRANSACTION_AND_SUBJECT: &str =
    include_str!("FIND_REVIEW_BY_TRANSACTION_AND_SUBJECT.sql");
