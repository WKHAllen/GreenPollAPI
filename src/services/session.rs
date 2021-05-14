use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services::User;

const NUM_USER_SESSIONS: i64 = 4;

pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub create_time: PrimitiveDateTime,
}

pub mod session_service {
    use super::*;

    pub async fn create_session(pool: &DBPool, user_id: i32) -> Result<String> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/create_session.sql", user_id)
            .fetch_all(pool).await,
            "Failed to create new session");

        delete_old_user_sessions(pool, user_id).await?;

        Ok(res.remove(0).id)
    }

    pub async fn session_exists(pool: &DBPool, session_id: String) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/get_session.sql", session_id)
            .fetch_all(pool).await,
            "Failed to check if session exists");

        Ok(res.len() == 1)
    }

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

    pub async fn get_user_sessions(pool: &DBPool, user_id: i32) -> Result<Vec<Session>> {
        let res = generic_service_err!(
            sqlx::query_file_as!(Session, "sql/session/get_user_sessions.sql", user_id)
            .fetch_all(pool).await,
            "Failed to get user sessions");

        Ok(res)
    }

    pub async fn delete_session(pool: &DBPool, session_id: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_session.sql", session_id)
            .fetch_all(pool).await,
            "Failed to delete session");

        Ok(())
    }

    pub async fn delete_user_sessions(pool: &DBPool, user_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_user_sessions.sql", user_id)
            .fetch_all(pool).await,
            "Failed to delete user sessions");

        Ok(())
    }

    pub async fn delete_old_user_sessions(pool: &DBPool, user_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/session/delete_old_user_sessions.sql", user_id, NUM_USER_SESSIONS)
            .fetch_all(pool).await,
            "Failed to delete old user sessions");

        Ok(())
    }
}
