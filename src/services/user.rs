extern crate bcrypt;

use std::io::{Error, ErrorKind, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::{generic_service_err, generic_err};
use crate::services;
use crate::services::Session;

/// Representation of the user database table
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub join_time: PrimitiveDateTime,
}

/// The user service
pub mod user_service {
    use super::*;

    /// Creates a user and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `username` - The user's username
    /// * `email` - The user's email
    /// * `password` - The user's password
    pub async fn create_user(pool: &DBPool, username: String, email: String, password: String) -> Result<User> {
        prune_unverified_users(pool).await?;

        let username_exists = user_exists_for_username(pool, username.clone()).await?;
        let email_exists = user_exists_for_email(pool, email.clone()).await?;

        if username_exists {
            generic_err!("Username is in use")
        } else if email_exists {
            generic_err!("Email is in use")
        } else if username.len() < 3 || username.len() > 63 {
            generic_err!("Username must be between 3 and 63 characters")
        } else if email.len() < 5 || email.len() > 63 {
            generic_err!("Email must be between 5 and 63 characters")
        } else if password.len() < 8 || password.len() > 255 {
            generic_err!("Password must be at least 8 characters")
        } else {
            let password_hash = generic_service_err!(
                hash(password, DEFAULT_COST),
                "Failed to hash password");

            let mut res = generic_service_err!(
                sqlx::query_file_as!(User, "sql/user/create_user.sql", username.clone(), email.clone(), password_hash)
                .fetch_all(pool).await,
                "Failed to create new user");

            Ok(res.remove(0))
        }
    }

    /// Returns whether or not a user exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn user_exists(pool: &DBPool, user_id: i32) -> Result<bool> {
        prune_unverified_users(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool).await,
            "Failed to check if user exists");

        Ok(res.len() == 1)
    }

    /// Returns whether or not a user exists given a username
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `username` - The username of the user
    pub async fn user_exists_for_username(pool: &DBPool, username: String) -> Result<bool> {
        prune_unverified_users(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user_by_username.sql", username)
            .fetch_all(pool).await,
            "Failed to check if user exists for username");

        Ok(res.len() == 1)
    }

    /// Returns whether or not a user exists given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address of the user
    pub async fn user_exists_for_email(pool: &DBPool, email: String) -> Result<bool> {
        prune_unverified_users(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to check if user exists for email");

        Ok(res.len() == 1)
    }

    /// Returns a user
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn get_user(pool: &DBPool, user_id: i32) -> Result<User> {
        prune_unverified_users(pool).await?;

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

    /// Returns a user given a username
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `username` - The username of the user
    pub async fn get_user_by_username(pool: &DBPool, username: String) -> Result<User> {
        prune_unverified_users(pool).await?;

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

    /// Returns a user given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address of the user
    pub async fn get_user_by_email(pool: &DBPool, email: String) -> Result<User> {
        prune_unverified_users(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/user/get_user_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to fetch user by email");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist"))
        }
    }

    /// Sets a user's username
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    /// * `username` - The new username
    pub async fn set_username(pool: &DBPool, user_id: i32, username: String) -> Result<()> {
        prune_unverified_users(pool).await?;

        let username_exists = user_exists_for_username(pool, username.clone()).await?;

        if username_exists {
            generic_err!("Username is in use")
        } else if username.len() < 3 || username.len() > 63 {
            generic_err!("Username must be between 3 and 63 characters")
        } else {
            generic_service_err!(
                sqlx::query_file!("sql/user/set_username.sql", username.clone(), user_id)
                .fetch_all(pool).await,
                "Failed to set username");

            Ok(())
        }
    }

    /// Sets the user's email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    /// * `email` - The new email address
    pub async fn set_email(pool: &DBPool, user_id: i32, email: String) -> Result<()> {
        prune_unverified_users(pool).await?;

        let email_exists = user_exists_for_email(pool, email.clone()).await?;

        if email_exists {
            generic_err!("Email is in use")
        } else if email.len() < 5 || email.len() > 63 {
            generic_err!("Email must be between 5 and 63 characters")
        } else {
            generic_service_err!(
                sqlx::query_file!("sql/user/set_email.sql", email.clone(), user_id)
                .fetch_all(pool).await,
                "Failed to set user email");

            Ok(())
        }
    }

    /// Sets the user's password
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    /// * `password` - The new password
    pub async fn set_password(pool: &DBPool, user_id: i32, password: String) -> Result<()> {
        prune_unverified_users(pool).await?;

        if password.len() < 8 || password.len() > 255 {
            generic_err!("Password must be at least 8 characters")
        } else {
            let password_hash = generic_service_err!(
                hash(password, DEFAULT_COST),
                "Failed to hash password");

            generic_service_err!(
                sqlx::query_file!("sql/user/set_password.sql", password_hash, user_id)
                .fetch_all(pool).await,
                "Failed to set user password");

            Ok(())
        }
    }

    /// Sets a user's verified status
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    /// * `verified` - The new verified status
    pub async fn set_verified(pool: &DBPool, user_id: i32, verified: bool) -> Result<()> {
        prune_unverified_users(pool).await?;

        generic_service_err!(
            sqlx::query_file!("sql/user/set_verified.sql", verified, user_id)
            .fetch_all(pool).await,
            "Failed to set user verified status");

        Ok(())
    }

    /// Deletes a user
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `user_id` - The ID of the user
    pub async fn delete_user(pool: &DBPool, user_id: i32) -> Result<()> {
        prune_unverified_users(pool).await?;

        generic_service_err!(
            sqlx::query_file!("sql/user/delete_user.sql", user_id)
            .fetch_all(pool).await,
            "Failed to delete user");

        Ok(())
    }

    /// Logs a user in and returns the new session
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The user's email address
    /// * `password` - The user's password
    pub async fn login(pool: &DBPool, email: String, password: String) -> Result<Session> {
        prune_unverified_users(pool).await?;

        let user_exists = user_exists_for_email(pool, email.clone()).await?;

        if user_exists {
            let user = get_user_by_email(pool, email.clone()).await?;

            let password_match = generic_service_err!(
                verify(password, &user.password[..]),
                "Failed to verify password hash");

            if password_match {
                let session = services::session_service::create_session(pool, user.id).await?;
                Ok(session)
            } else {
                Err(Error::new(ErrorKind::Other, "Invalid login"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid login"))
        }
    }

    /// Prunes all old and unverified user records
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    pub async fn prune_unverified_users(pool: &DBPool) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/user/prune_unverified_users.sql")
            .fetch_all(pool).await,
            "Failed to prune unverified users");

        Ok(())
    }
}
