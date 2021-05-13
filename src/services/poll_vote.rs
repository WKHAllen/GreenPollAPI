use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;

pub struct PollVote {
    pub id: i32,
    pub user_id: i32,
    pub poll_id: i32,
    pub poll_option_id: i32,
    pub vote_time: PrimitiveDateTime,
}

pub mod poll_vote_service {
    use super::*;


}
