extern crate bcrypt;

use std::io::{Error, ErrorKind, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::types::time::{PrimitiveDateTime};
use crate::util::DBPool;

#[allow(dead_code)]
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

    pub async fn create_user(pool: &DBPool, email: String, password: String) -> Result<i32> {
        let password_hash = match hash(password, DEFAULT_COST) {
            Ok(pw_hash) => Ok(pw_hash),
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to hash password"))
        }?;

        let res = match sqlx::query_file_as!(User, "sql/user/create_user.sql", email, password_hash)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to create new user"))
            }?;

        Ok(res[0].id)
    }

    pub async fn user_exists(pool: &DBPool, user_id: i32) -> Result<bool> {
        let res = match sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to check if user exists"))
            }?;

        Ok(res.len() == 1)
    }

    pub async fn get_user(pool: &DBPool, user_id: i32) -> Result<User> {
        let mut res = match sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to fetch user"))
            }?;

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist"))
        }
    }

    pub async fn get_user_by_username(pool: &DBPool, username: String) -> Result<User> {
        let mut res = match sqlx::query_file_as!(User, "sql/user/get_user_by_username.sql", username)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to fetch user by username"))
            }?;

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(Error::new(ErrorKind::Other, "User does not exist"))
        }
    }

    pub async fn set_username(pool: &DBPool, user_id: i32, username: String) -> Result<()> {
        match sqlx::query_file!("sql/user/set_username.sql", username, user_id)
            .fetch_all(pool)
            .await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to set username"))
            }?;

        Ok(())
    }

    pub async fn set_email(pool: &DBPool, user_id: i32, email: String) -> Result<()> {
        match sqlx::query_file!("sql/user/set_email.sql", email, user_id)
            .fetch_all(pool)
            .await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to set user email"))
            }?;

        Ok(())
    }

    pub async fn set_password(pool: &DBPool, user_id: i32, password: String) -> Result<()> {
        let password_hash = match hash(password, DEFAULT_COST) {
            Ok(pw_hash) => Ok(pw_hash),
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to hash password"))
        }?;

        match sqlx::query_file!("sql/user/set_password.sql", password_hash, user_id)
            .fetch_all(pool)
            .await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to set user password"))
            }?;

        Ok(())
    }

    pub async fn delete_user(pool: &DBPool, user_id: i32) -> Result<()> {
        match sqlx::query_file!("sql/user/delete_user.sql", user_id)
            .fetch_all(pool)
            .await {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(ErrorKind::Other, "Failed to delete user"))
            }?;

        Ok(())
    }
}
