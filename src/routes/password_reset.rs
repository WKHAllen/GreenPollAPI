use actix_web::{HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use std::io::{Error, ErrorKind};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, ExistsJSON, success_json, FRONTEND_URL};
use crate::emailer;

/// Query parameters for requesting a password reset
#[derive(Serialize, Deserialize)]
pub struct RequestPasswordResetQuery {
    email: String,
}

/// Query parameters for checking if a password reset record exists
#[derive(Serialize, Deserialize)]
pub struct PasswordResetExistsQuery {
    reset_id: String,
}

/// Query parameters for resetting a password
#[derive(Serialize, Deserialize)]
pub struct ResetPasswordQuery {
    reset_id: String,
    new_password: String,
}

/// The password reset routes
pub mod password_reset_routes {
    use super::*;

    /// Requests a password reset
    #[get("/request_password_reset")]
    pub async fn request_password_reset(
        query: web::Query<RequestPasswordResetQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let password_reset = generic_http_err!(
            services::password_reset_service::create_password_reset(&data.pool, query.email.clone())
            .await);

        match emailer::send_formatted_email(
            query.email.clone(),
            "GreenPoll - Password Reset".to_string(),
            "password_reset".to_string(),
            [("url", FRONTEND_URL), ("reset_id", &password_reset.id[..])].iter().cloned().collect()
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to send password reset email"))
        }?;

        Ok(success_json())
    }

    /// Checks whether or not a password reset record exists
    #[get("/password_reset_exists")]
    pub async fn password_reset_exists(
        query: web::Query<PasswordResetExistsQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let exists = generic_http_err!(
            services::password_reset_service::password_reset_exists(&data.pool, query.reset_id.clone())
            .await);

        Ok(HttpResponse::Ok().json(ExistsJSON {
            exists: exists
        }))
    }

    /// Resets a password
    #[get("/reset_password")]
    pub async fn reset_password(
        query: web::Query<ResetPasswordQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        generic_http_err!(
            services::password_reset_service::reset_password(&data.pool, query.reset_id.clone(), query.new_password.clone())
            .await);

        Ok(success_json())
    }
}
