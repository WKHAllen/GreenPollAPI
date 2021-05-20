use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services;
use crate::services::User;

/// Representation of the password reset database table
pub struct PasswordReset {
    pub id: String,
    pub email: String,
    pub create_time: PrimitiveDateTime,
}

/// The password reset service
pub mod password_reset_service {
    use super::*;

    /// Creates a password reset record and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address of the user requesting the password reset
    pub async fn create_password_reset(pool: &DBPool, email: String) -> Result<PasswordReset> {
        prune_password_resets(pool).await?;

        let exists = password_reset_exists_for_email(pool, email.clone()).await?;

        if !exists {
            let mut res = generic_service_err!(
                sqlx::query_file_as!(PasswordReset, "sql/password_reset/create_password_reset.sql", email.clone())
                .fetch_all(pool).await,
                "Failed to create new password reset record");

            Ok(res.remove(0))
        } else {
            let password_reset = get_password_reset_for_email(pool, email.clone()).await?;

            Ok(password_reset)
        }
    }

    /// Returns whether or not a password reset record exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `password_reset_id` - The ID of the password reset record
    pub async fn password_reset_exists(pool: &DBPool, password_reset_id: String) -> Result<bool> {
        prune_password_resets(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to check if password reset record exists");

        Ok(res.len() == 1)
    }

    /// Returns whether or not a password reset record exists given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address associated with the password reset record
    pub async fn password_reset_exists_for_email(pool: &DBPool, email: String) -> Result<bool> {
        prune_password_resets(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to check if password reset record exists for given email");

        Ok(res.len() == 1)
    }

    /// Returns a password reset record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `password_reset_id` - The ID of the password reset record
    pub async fn get_password_reset(pool: &DBPool, password_reset_id: String) -> Result<PasswordReset> {
        prune_password_resets(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to fetch password reset record");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Password reset record does not exist"))
        }
    }

    /// Returns a password reset record given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address associated with the password reset record
    pub async fn get_password_reset_for_email(pool: &DBPool, email: String) -> Result<PasswordReset> {
        prune_password_resets(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to fetch password reset record for given email");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Password reset record does not exist for given email"))
        }
    }

    /// Returns the user who created the password reset record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `password_reset_id` - The ID of the password reset record
    pub async fn get_user_by_password_reset(pool: &DBPool, password_reset_id: String) -> Result<User> {
        prune_password_resets(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/password_reset/get_user_by_password_reset_id.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to fetch user by password reset ID");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist for given password reset ID"))
        }
    }

    /// Deletes a password reset record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `password_reset_id` - The ID of the password reset record
    pub async fn delete_password_reset(pool: &DBPool, password_reset_id: String) -> Result<()> {
        prune_password_resets(pool).await?;

        generic_service_err!(
            sqlx::query_file!("sql/password_reset/delete_password_reset.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to delete password reset record");

        Ok(())
    }

    /// Resets a user's password and deletes the password reset record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `password_reset_id` - The ID of the password reset record
    /// * `new_password` - The user's new password
    pub async fn reset_password(pool: &DBPool, password_reset_id: String, new_password: String) -> Result<()> {
        prune_password_resets(pool).await?;

        let valid = password_reset_exists(pool, password_reset_id.clone()).await?;

        if valid {
            let user = get_user_by_password_reset(pool, password_reset_id.clone()).await?;
            delete_password_reset(pool, password_reset_id.clone()).await?;
            services::user_service::set_password(pool, user.id, new_password).await?;

            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid password reset ID"))
        }
    }

    /// Prunes all old password reset records
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    pub async fn prune_password_resets(pool: &DBPool) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/password_reset/prune_password_resets.sql")
            .fetch_all(pool).await,
            "Failed to prune password reset records");

        Ok(())
    }
}
