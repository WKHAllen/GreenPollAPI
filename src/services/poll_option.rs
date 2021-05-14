use std::io::{Error, ErrorKind, Result};
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services::Poll;

pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub value: String,
}

pub mod poll_option_service {
    use super::*;

    pub async fn create_poll_option(pool: &DBPool, poll_id: i32, value: String) -> Result<i32> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll_option/create_poll_option.sql", poll_id, value)
            .fetch_all(pool).await,
            "Failed to create new poll option");

        Ok(res[0].id)
    }

    pub async fn poll_option_exists(pool: &DBPool, poll_option_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll_option/get_poll_option.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to check if poll option exists");

        Ok(res.len() == 1)
    }

    pub async fn get_poll_option(pool: &DBPool, poll_option_id: i32) -> Result<PollOption> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll_option/get_poll_option.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to fetch poll option");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Poll option does not exist"))
        }
    }

    pub async fn get_poll_option_poll(pool: &DBPool, poll_option_id: i32) -> Result<Poll> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll_option/get_poll_option_poll.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to fetch poll option poll");

        Ok(res.remove(0))
    }

    pub async fn set_poll_option_value(pool: &DBPool, poll_option_id: i32, value: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll_option/set_poll_option_value.sql", value, poll_option_id)
            .fetch_all(pool).await,
            "Failed to set poll option value");

        Ok(())
    }

    pub async fn delete_poll_option(pool: &DBPool, poll_option_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll_option/delete_poll_option.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to delete poll option");

        Ok(())
    }
}