use std::io::{Error, ErrorKind, Result};
use sqlx::types::time::PrimitiveDateTime;
use crate::util::DBPool;
use crate::generic_service_err;
use crate::services;
use crate::services::User;

/// Representation of the verify database table
pub struct Verify {
    pub id: String,
    pub email: String,
    pub create_time: PrimitiveDateTime,
}

/// The verify service
pub mod verify_service {
    use super::*;

    /// Creates a verification record and returns the resulting record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address of the user being verified
    pub async fn create_verification(pool: &DBPool, email: String) -> Result<Verify> {
        prune_verifications(pool).await?;

        let exists = verification_exists_for_email(pool, email.clone()).await?;

        if !exists {
            let mut res = generic_service_err!(
                sqlx::query_file_as!(Verify, "sql/verify/create_verification.sql", email.clone())
                .fetch_all(pool).await,
                "Failed to create new verification record");

            Ok(res.remove(0))
        } else {
            let verification = get_verification_for_email(pool, email.clone()).await?;

            Ok(verification)
        }
    }

    /// Returns whether or not a verification record exists
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `verify_id` - The ID of the verification record
    pub async fn verification_exists(pool: &DBPool, verify_id: String) -> Result<bool> {
        prune_verifications(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(Verify, "sql/verify/get_verification.sql", verify_id)
            .fetch_all(pool).await,
            "Failed to check if verification record exists");

        Ok(res.len() == 1)
    }

    /// Returns whether or not a verification record exists given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address associated with the verification record
    pub async fn verification_exists_for_email(pool: &DBPool, email: String) -> Result<bool> {
        prune_verifications(pool).await?;

        let res = generic_service_err!(
            sqlx::query_file_as!(Verify, "sql/verify/get_verification_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to check if verification record exists for given email");

        Ok(res.len() == 1)
    }

    /// Returns a verification record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `verify_id` - The ID of the verification record
    pub async fn get_verification(pool: &DBPool, verify_id: String) -> Result<Verify> {
        prune_verifications(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(Verify, "sql/verify/get_verification.sql", verify_id)
            .fetch_all(pool).await,
            "Failed to fetch verification record");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Verification record does not exist"))
        }
    }

    /// Returns a verification record given an email address
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `email` - The email address associated with the verification record
    pub async fn get_verification_for_email(pool: &DBPool, email: String) -> Result<Verify> {
        prune_verifications(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(Verify, "sql/verify/get_verification_by_email.sql", email)
            .fetch_all(pool).await,
            "Failed to fetch verification record for given email");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "Verification record does not exist for given email"))
        }
    }

    /// Returns the user who created the verification record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `verify_id` - The ID of the verification record
    pub async fn get_user_by_verification(pool: &DBPool, verify_id: String) -> Result<User> {
        prune_verifications(pool).await?;

        let mut res = generic_service_err!(
            sqlx::query_file_as!(User, "sql/verify/get_user_by_verify_id.sql", verify_id)
            .fetch_all(pool).await,
            "Failed to fetch user by verify ID");

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist for given verify ID"))
        }
    }

    /// Deletes a verification record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `verify_id` - The ID of the verification record
    pub async fn delete_verification(pool: &DBPool, verify_id: String) -> Result<()> {
        prune_verifications(pool).await?;

        generic_service_err!(
            sqlx::query_file!("sql/verify/delete_verification.sql", verify_id)
            .fetch_all(pool).await,
            "Failed to delete verification record");

        Ok(())
    }

    /// Verifies a user's account and deletes the verification record
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    /// * `verify_id` - The ID of the verification record
    pub async fn verify_user(pool: &DBPool, verify_id: String) -> Result<()> {
        prune_verifications(pool).await?;

        let valid = verification_exists(pool, verify_id.clone()).await?;

        if valid {
            let user = get_user_by_verification(pool, verify_id.clone()).await?;
            delete_verification(pool, verify_id.clone()).await?;
            services::user_service::set_verified(pool, user.id, true).await?;

            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Invalid verify ID"))
        }
    }

    /// Prunes all old verification records
    /// 
    /// # Arguments
    /// 
    /// * `pool` - The database pool
    pub async fn prune_verifications(pool: &DBPool) -> Result<()> {
        generic_service_err!(
            sqlx::query_file!("sql/verify/prune_verifications.sql")
            .fetch_all(pool).await,
            "Failed to prune verification records");

        Ok(())
    }
}
