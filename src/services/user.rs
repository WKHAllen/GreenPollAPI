extern crate bcrypt;

use std::io::{Error, ErrorKind, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services;
use crate::services::Session;

pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub join_time: PrimitiveDateTime,
}

pub mod user_service {
    use super::*;

    pub async fn create_user(pool: &DBPool, email: String, password: String) -> Result<User> {
        let password_hash = generic_service_err!(
            hash(password, DEFAULT_COST),
            "Failed to hash password");

        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/create_user.sql", email, password_hash)
            .fetch_all(pool).await,
            "Failed to create new user");

        Ok(res.remove(0))
    }

    pub async fn user_exists(pool: &DBPool, user_id: i32) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool).await,
            "Failed to check if user exists");

        Ok(res.len() == 1)
    }

    pub async fn get_user(pool: &DBPool, user_id: i32) -> Result<User> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool).await,
            "Failed to fetch user");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist"))
        }
    }

    pub async fn get_user_by_username(pool: &DBPool, username: String) -> Result<User> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user_by_username.sql", username)
            .fetch_all(pool).await,
            "Failed to fetch user by username");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist"))
        }
    }

    pub async fn set_username(pool: &DBPool, user_id: i32, username: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/user/set_username.sql", username, user_id)
            .fetch_all(pool).await,
            "Failed to set username");

        Ok(())
    }

    pub async fn set_email(pool: &DBPool, user_id: i32, email: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/user/set_email.sql", email, user_id)
            .fetch_all(pool).await,
            "Failed to set user email");

        Ok(())
    }

    pub async fn set_password(pool: &DBPool, user_id: i32, password: String) -> Result<()> {
        let password_hash = generic_service_err!(
            hash(password, DEFAULT_COST),
            "Failed to hash password");

        generic_service_err!(
            sqlx::query_file!("sql/user/set_password.sql", password_hash, user_id)
            .fetch_all(pool).await,
            "Failed to set user password");

        Ok(())
    }

    pub async fn set_verified(pool: &DBPool, user_id: i32, verified: bool) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/user/set_verified.sql", verified, user_id)
            .fetch_all(pool).await,
            "Failed to set user verified status");

        Ok(())
    }

    pub async fn delete_user(pool: &DBPool, user_id: i32) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/user/delete_user.sql", user_id)
            .fetch_all(pool).await,
            "Failed to delete user");

        Ok(())
    }

    pub async fn login(pool: &DBPool, user_id: i32, password: String) -> Result<Session> {
        let user_exists = user_exists(pool, user_id).await?;

        if user_exists {
            let user = get_user(pool, user_id).await?;

            let password_match = generic_service_err!(
                verify(password, &user.password[..]),
                "Failed to verify password hash");

            if password_match {
                let session = services::session_service::create_session(pool, user_id).await?;
                Ok(session)
            } else {
                Err(Error::new(ErrorKind::Other, "Invalid login"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid login"))
        }
    }
}
