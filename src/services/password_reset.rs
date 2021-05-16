use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services;
use crate::services::User;

pub struct PasswordReset {
    pub id: String,
    pub email: String,
    pub create_time: PrimitiveDateTime,
}

pub mod password_reset_service {
    use super::*;

    pub async fn create_password_reset(pool: &DBPool, email: String) -> Result<PasswordReset> {
        let mut res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/create_password_reset.sql", email)
            .fetch_all(pool).await,
            "Failed to create new password reset record");

        Ok(res.remove(0))
    }

    pub async fn password_reset_exists(pool: &DBPool, password_reset_id: String) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to check if password reset record exists");

        Ok(res.len() == 1)
    }

    pub async fn password_reset_exists_for_email(pool: &DBPool, email: String) -> Result<bool> {
        let res = generic_service_err!(
            sqlx::query_file_as!(PasswordReset, "sql/password_reset/get_password_reset_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to check if password reset record exists for given email");

        Ok(res.len() == 1)
    }

    pub async fn get_password_reset(pool: &DBPool, password_reset_id: String) -> Result<PasswordReset> {
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

    pub async fn get_password_reset_for_email(pool: &DBPool, email: String) -> Result<PasswordReset> {
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

    pub async fn get_user_by_password_reset(pool: &DBPool, password_reset_id: String) -> Result<User> {
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

    pub async fn delete_password_reset(pool: &DBPool, password_reset_id: String) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/password_reset/delete_password_reset.sql", password_reset_id)
            .fetch_all(pool).await,
            "Failed to delete password reset record");

        Ok(())
    }

    pub async fn reset_password(pool: &DBPool, password_reset_id: String, new_password: String) -> Result<()> {
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
}
