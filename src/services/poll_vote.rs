use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services;
use crate::services::Poll;

pub struct PollVote {
    pub id: i32,
    pub user_id: i32,
    pub poll_id: i32,
    pub poll_option_id: i32,
    pub vote_time: PrimitiveDateTime,
}

pub mod poll_vote_service {
    use super::*;

    pub async fn poll_vote_exists(pool: &DBPool, user_id: i32, poll_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll_vote/get_poll_vote.sql", user_id, poll_id)
            .fetch_all(pool).await,
            "Failed to check if poll vote exists");

        Ok(res.len() == 1)
    }

    pub async fn get_poll_vote(pool: &DBPool, user_id: i32, poll_id: i32) -> Result<PollVote> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll_vote/get_poll_vote.sql", user_id, poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll vote");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Poll vote does not exist"))
        }
    }

    pub async fn get_poll_vote_by_vote_id(pool: &DBPool, poll_vote_id: i32) -> Result<PollVote> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll_vote/get_poll_vote_by_vote_id.sql", poll_vote_id)
            .fetch_all(pool).await,
            "Failed to fetch poll vote by vote ID");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Poll vote does not exist"))
        }
    }

    pub async fn get_poll_vote_poll(pool: &DBPool, poll_vote_id: i32) -> Result<Poll> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll_vote/get_poll_vote_poll.sql", poll_vote_id)
            .fetch_all(pool).await,
            "Failed to fetch poll vote poll");

        Ok(res.remove(0))
    }

    pub async fn vote(pool: &DBPool, user_id: i32, poll_option_id: i32) -> Result<PollVote> {
        let poll = services::poll_option_service::get_poll_option_poll(pool, poll_option_id).await?;

        unvote(pool, user_id, poll.id).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll_vote/vote.sql", user_id, poll.id, poll_option_id)
            .fetch_all(pool).await,
            "Failed to vote on poll");

        Ok(res.remove(0))
    }

    pub async fn unvote(pool: &DBPool, user_id: i32, poll_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll_vote/unvote.sql", user_id, poll_id)
            .fetch_all(pool).await,
            "Failed to remove vote from poll");

        Ok(())
    }

    pub async fn unvote_by_poll_option_id(pool: &DBPool, user_id: i32, poll_option_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll_vote/unvote_by_poll_option_id.sql", user_id, poll_option_id)
            .fetch_all(pool).await,
            "Failed to remove vote from poll");

        Ok(())
    }
}
