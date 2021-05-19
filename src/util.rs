use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result};
use crate::services;
use crate::services::User;

pub type DBPool = sqlx::Pool<sqlx::Postgres>;

pub struct AppData {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessJSON {
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorJSON {
    pub error: String,
}

#[macro_export]
macro_rules! generic_http_err {
    ( $x:expr ) => {
        match $x {
            Ok(val) => Ok(val),
            Err(e) => Err(HttpResponse::Ok().json(ErrorJSON {
                error: format!("{}", e)
            })),
        }?
    };
}

#[macro_export]
macro_rules! generic_service_err {
    ( $x:expr, $err:literal ) => {
        match $x {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::new(ErrorKind::Other, $err))
        }?
    };
}

#[macro_export]
macro_rules! generic_err {
    ( $err:literal ) => {
        Err(Error::new(ErrorKind::Other, $err))
    };
}

pub fn success_json() -> HttpResponse {
    HttpResponse::Ok().json(SuccessJSON {
        success: true
    })
}

pub fn error_json(err: &str) -> HttpResponse {
    HttpResponse::Ok().json(ErrorJSON {
        error: String::from(err)
    })
}

pub async fn get_user_by_session(pool: &DBPool, req: HttpRequest) -> Result<User> {
    let session_id = match req.cookie("session_id") {
        Some(val) => Ok(val),
        None => Err(HttpResponse::Ok().json(ErrorJSON {
            error: String::from("Not logged in")
        }))
    }?;

    let user = generic_http_err!(
        services::session_service::get_user_by_session_id(pool, String::from(session_id.value()))
        .await);

    Ok(user)
}
