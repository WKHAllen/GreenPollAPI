use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services::User;

/// The maximum number of user sessions
const NUM_USER_SESSIONS: i64 = 4;

/// Representation of the session database table
pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub create_time: PrimitiveDateTime,
}

/// The session service
pub mod session_service {
    use super::*;

    /// Creates a user session and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user creating the session
    pub async fn create_session(pool: &DBPool, user_id: i32) -> Result<Session> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/create_session.sql", user_id)
            .fetch_all(pool).await,
            "Failed to create new session");

        delete_old_user_sessions(pool, user_id).await?;

        Ok(res.remove(0))
    }

    /// Returns whether or not a session exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `session_id` - The ID of the session
    pub async fn session_exists(pool: &DBPool, session_id: String) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/get_session.sql", session_id)
            .fetch_all(pool).await,
            "Failed to check if session exists");

        Ok(res.len() == 1)
    }

    /// Returns a user session record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `session_id` - The ID of the session
    pub async fn get_session(pool: &DBPool, session_id: String) -> Result<Session> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/get_session.sql", session_id)
            .fetch_all(pool).await,
            "Failed to fetch session");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Session does not exist"))
        }
    }

    /// Returns the user associated with the session
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `session_id` - The ID of the session
    pub async fn get_user_by_session_id(pool: &DBPool, session_id: String) -> Result<User> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/session/get_user_by_session_id.sql", session_id)
            .fetch_all(pool).await,
            "Failed to fetch user with session ID");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User or session does not exist"))
        }
    }

    /// Returns all sessions associated with a user
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn get_user_sessions(pool: &DBPool, user_id: i32) -> Result<Vec<Session>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/get_user_sessions.sql", user_id)
            .fetch_all(pool).await,
            "Failed to get user sessions");

        Ok(res)
    }

    /// Deletes a user session
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `session_id` - The ID of the session
    pub async fn delete_session(pool: &DBPool, session_id: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_session.sql", session_id)
            .fetch_all(pool).await,
            "Failed to delete session");

        Ok(())
    }

    /// Deletes all sessions associated with a user
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn delete_user_sessions(pool: &DBPool, user_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_user_sessions.sql", user_id)
            .fetch_all(pool).await,
            "Failed to delete user sessions");

        Ok(())
    }

    /// Deletes all old user sessions
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn delete_old_user_sessions(pool: &DBPool, user_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_old_user_sessions.sql", user_id, NUM_USER_SESSIONS)
            .fetch_all(pool).await,
            "Failed to delete old user sessions");

        Ok(())
    }
}
