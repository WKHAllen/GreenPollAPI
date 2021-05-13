use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;

pub struct Poll {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub create_time: PrimitiveDateTime,
}

pub mod poll_service {
    use super::*;

    pub async fn create_poll(pool: &DBPool, user_id: i32, title: String, description: String) -> Result<i32> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll/create_poll.sql", user_id, title, description)
            .fetch_all(pool).await,
            "Failed to create new poll");

        Ok(res[0].id)
    }
}
