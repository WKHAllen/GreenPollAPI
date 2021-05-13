use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;

pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub value: String,
}

pub mod poll_option_service {
    use super::*;


}
