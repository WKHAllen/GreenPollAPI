use std::io::{Error, ErrorKind, Result};
use crate::util::DBPool;
use crate::{generic_service_err, generic_err};
use crate::services;
use crate::services::Poll;

/// The maximum number of options per poll
const NUM_POLL_OPTIONS: usize = 16;

/// Representation of the poll option database table
pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub value: String,
}

/// The poll option service
pub mod poll_option_service {
    use super::*;

    /// Creates a poll option and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    /// * `value` - The text representing the poll option
    pub async fn create_poll_option(pool: &DBPool, poll_id: i32, value: String) -> Result<PollOption> {
        let num_poll_options = get_num_poll_options(pool, poll_id).await?;

        if num_poll_options >= NUM_POLL_OPTIONS {
            generic_err!("Maximum number of poll options has been reached")
        } else if value.len() < 1 || value.len() > 255 {
            generic_err!("Option value must be between 1 and 255 characters")
        } else {
            let mut res = generic_service_err!(
                sqlx::query_file_as!(PollOption, "sql/poll_option/create_poll_option.sql", poll_id, value)
                .fetch_all(pool).await,
                "Failed to create new poll option");

            Ok(res.remove(0))
        }
    }

    /// Returns whether or not a poll option exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_option_id` - The ID of the poll option
    pub async fn poll_option_exists(pool: &DBPool, poll_option_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PollOption, "sql/poll_option/get_poll_option.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to check if poll option exists");

        Ok(res.len() == 1)
    }

    /// Returns a poll option
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_option_id` - The ID of the poll option
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

    /// Returns the poll associated with the poll option
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_option_id` - The ID of the poll option
    pub async fn get_poll_option_poll(pool: &DBPool, poll_option_id: i32) -> Result<Poll> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Poll, "sql/poll_option/get_poll_option_poll.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to fetch poll option poll");

        Ok(res.remove(0))
    }

    /// Sets the text representing the poll option
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_option_id` - The ID of the poll option
    /// * `value` - The new text representing the poll option
    pub async fn set_poll_option_value(pool: &DBPool, poll_option_id: i32, value: String) -> Result<()> {
        if value.len() < 1 || value.len() > 255 {
            generic_err!("Option value must be between 1 and 255 characters")
        } else {
            generic_service_err!(
                sqlx::query_file!("sql/poll_option/set_poll_option_value.sql", value, poll_option_id)
                .fetch_all(pool).await,
                "Failed to set poll option value");

            Ok(())
        }
    }

    /// Returns the number of options for a given poll
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_id` - The ID of the poll
    pub async fn get_num_poll_options(pool: &DBPool, poll_id: i32) -> Result<usize> {
        let poll_options = services::poll_service::get_poll_options(pool, poll_id).await?;

        Ok(poll_options.len())
    }

    /// Deletes a poll option
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `poll_option_id` - The ID of the poll option
    pub async fn delete_poll_option(pool: &DBPool, poll_option_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/poll_option/delete_poll_option.sql", poll_option_id)
            .fetch_all(pool).await,
            "Failed to delete poll option");

        Ok(())
    }
}
