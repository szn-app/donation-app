use std::fs;
use std::path::Path;

pub const GET_LOCATIONS: &str = include_str!("GET_LOCATIONS.sql");
pub const GET_LOCATION_BY_ID: &str = include_str!("GET_LOCATION_BY_ID.sql");
pub const GET_LOCATIONS_BY_PROFILE: &str = include_str!("GET_LOCATIONS_BY_PROFILE.sql");
pub const ADD_LOCATION: &str = include_str!("ADD_LOCATION.sql");
pub const UPDATE_LOCATION: &str = include_str!("UPDATE_LOCATION.sql");
pub const DELETE_LOCATION: &str = include_str!("DELETE_LOCATION.sql");
