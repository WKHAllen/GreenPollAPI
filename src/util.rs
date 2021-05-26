use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result};
use crate::services;
use crate::services::User;

/// The URL for the frontend
pub const FRONTEND_URL: &str = "https://greenpoll.herokuapp.com/";

/// Shortcut for the sqlx postgres pool type
pub type DBPool = sqlx::Pool<sqlx::Postgres>;

/// Container for the database pool within the actix app
pub struct AppData {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

/// Success JSON message
#[derive(Serialize, Deserialize)]
pub struct SuccessJSON {
    pub success: bool,
}

/// Error JSON message
#[derive(Serialize, Deserialize)]
pub struct ErrorJSON {
    pub error: String,
}

/// A macro for matching errors at the routing layer and returning them in JSON in a generic format
/// 
/// # Arguments
/// 
/// * `x` - The code to be matched for errors
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

/// A macro for matching errors at the service layer and returning them in a generic error object
/// 
/// # Arguments
/// 
/// * `x` - The code to be matched for errors
/// * `err` - The error message
#[macro_export]
macro_rules! generic_service_err {
    ( $x:expr, $err:literal ) => {
        match $x {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::new(ErrorKind::Other, $err))
        }?
    };
}

/// A macro for creating generic errors
/// 
/// # Arguments
/// 
/// * `err` - The error message
#[macro_export]
macro_rules! generic_err {
    ( $err:literal ) => {
        Err(Error::new(ErrorKind::Other, $err))
    };
}

/// Returns a success JSON HTTP response
pub fn success_json() -> HttpResponse {
    HttpResponse::Ok().json(SuccessJSON {
        success: true
    })
}

/// Returns an error JSON HTTP response
/// 
/// # Arguments
/// 
/// * `err` - The error message
pub fn error_json(err: &str) -> HttpResponse {
    HttpResponse::Ok().json(ErrorJSON {
        error: String::from(err)
    })
}

/// Returns the user that is logged in
/// 
/// # Arguments
/// 
/// * `pool` - The database pool
/// * `req` - The HTTP request object
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
