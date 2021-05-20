use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::{generic_service_err, generic_err};
use crate::services::{PollOption, PollVote};

/// Representation of the poll database table
pub struct Poll {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub create_time: PrimitiveDateTime,
}

/// The poll service
pub mod poll_service {
    use super::*;

    /// Creates a poll and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user creating the poll
    /// * `title` - The poll title
    /// * `description` - The poll description
    pub async fn create_poll(pool: &DBPool, user_id: i32, title: String, description: String) -> Result<Poll> {
        if title.len() < 1 || title.len() > 255 {
            generic_err!("Title must be between 1 and 255 characters")
        } else if description.len() > 1023 {
            generic_err!("Description must be no more than 1023 characters")
        } else {
            let mut res = generic_service_err!(
                sqlx::query_file_as!(Poll, "sql/poll/create_poll.sql", user_id, title, description)
                .fetch_all(pool).await,
                "Failed to create new poll");

            Ok(res.remove(0))
        }
    }

    /// Returns whether or not a poll exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    pub async fn poll_exists(pool: &DBPool, poll_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll/get_poll.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to check if poll exists");

        Ok(res.len() == 1)
    }

    /// Returns a poll
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
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

    /// Returns all options associated with a poll
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    pub async fn get_poll_options(pool: &DBPool, poll_id: i32) -> Result<Vec<PollOption>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll/get_poll_options.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll options");

        Ok(res)
    }

    /// Returns all votes associated with a poll
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    pub async fn get_poll_votes(pool: &DBPool, poll_id: i32) -> Result<Vec<PollVote>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollVote, "sql/poll/get_poll_votes.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to fetch poll votes");

        Ok(res)
    }

    /// Sets the poll title
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    /// * `title` - The new poll title
    pub async fn set_title(pool: &DBPool, poll_id: i32, title: String) -> Result<()> {
        if title.len() < 1 || title.len() > 255 {
            generic_err!("Title must be between 1 and 255 characters")
        } else {
            generic_service_err!(
                sqlx::query_file!("sql/poll/set_title.sql", title, poll_id)
                .fetch_all(pool).await,
                "Failed to set poll title");

            Ok(())
        }
    }

    /// Sets the poll description
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    /// * `description` - The new poll description
    pub async fn set_description(pool: &DBPool, poll_id: i32, description: String) -> Result<()> {
        if description.len() > 1023 {
            generic_err!("Description must be no more than 1023 characters")
        } else {
            generic_service_err!(
                sqlx::query_file!("sql/poll/set_description.sql", description, poll_id)
                .fetch_all(pool).await,
                "Failed to set poll description");

            Ok(())
        }
    }

    /// Deletes a poll
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    pub async fn delete_poll(pool: &DBPool, poll_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll/delete_poll.sql", poll_id)
            .fetch_all(pool).await,
            "Failed to delete poll");

        Ok(())
    }
}
