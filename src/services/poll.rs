use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services::{PollOption, PollVote};

pub struct Poll {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub create_time: PrimitiveDateTime,
}

pub mod poll_service {
    use super::*;

    pub async fn create_poll(pool: &DBPool, user_id: i32, title: String, description: String) -> Result<Poll> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll/create_poll.sql", user_id, title, description)
            .fetch_all(pool).await,
            "Failed to create new poll");

        Ok(res.remove(0))
    }

    pub async fn poll_exists(pool: &DBPool, poll_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll/get_poll.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to check if poll exists");

        Ok(res.len() == 1)
    }

    pub async fn get_poll(pool: &DBPool, poll_id: i32) -> Result<Poll> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll/get_poll.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Poll does not exist"))
        }
    }

    pub async fn get_poll_options(pool: &DBPool, poll_id: i32) -> Result<Vec<PollOption>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll/get_poll_options.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll options");

        Ok(res)
    }

    pub async fn get_poll_votes(pool: &DBPool, poll_id: i32) -> Result<Vec<PollVote>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll/get_poll_votes.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll votes");

        Ok(res)
    }

    pub async fn set_title(pool: &DBPool, poll_id: i32, title: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll/set_title.sql", title, poll_id)
            .fetch_all(pool).await,
            "Failed to set poll title");

        Ok(())
    }

    pub async fn set_description(pool: &DBPool, poll_id: i32, description: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll/set_description.sql", description, poll_id)
            .fetch_all(pool).await,
            "Failed to set poll description");

        Ok(())
    }

    pub async fn delete_poll(pool: &DBPool, poll_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll/delete_poll.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to delete poll");

        Ok(())
    }
}
