extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::types::time::{PrimitiveDateTime};
use crate::dbinit::DBPool;

#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub verified: bool,
    pub join_time: PrimitiveDateTime,
}

pub mod user_service {
    use super::*;

    pub async fn create_user(pool: &DBPool, email: String, password: String) -> std::io::Result<i32> {
        let password_hash = match hash(password, DEFAULT_COST) {
            Ok(pw_hash) => Ok(pw_hash),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to hash password"))
        }?;

        let res = match sqlx::query_file_as!(User, "sql/user/create_user.sql", email, password_hash)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create new user"))
            }?;

        Ok(res[0].id)
    }

    pub async fn get_user(pool: &DBPool, user_id: i32) -> std::io::Result<User> {
        let mut res = match sqlx::query_file_as!(User, "sql/user/get_user.sql", user_id)
            .fetch_all(pool)
            .await {
                Ok(res) => Ok(res),
                Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch user"))
            }?;

        if res.len() == 1 {
            Ok(res.remove(0))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "User does not exist"))
        }
    }
}
