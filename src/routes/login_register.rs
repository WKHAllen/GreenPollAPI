use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result, web, get};
use actix_web::cookie::{Cookie, SameSite};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use std::io::{Error, ErrorKind};
use crate::{services, generic_http_err};
use crate::util::{AppData, SuccessJSON, ErrorJSON, success_json, get_user_by_session, FRONTEND_URL};
use crate::emailer;

/// Query parameters for registration
#[derive(Serialize, Deserialize)]
pub struct RegisterQuery {
    username: String,
    email: String,
    password: String,
}

/// Query parameters for logging in
#[derive(Serialize, Deserialize)]
pub struct LoginQuery {
    email: String,
    password: String,
}

/// The login and registration routes
pub mod login_register_routes {
    use super::*;

    /// Registers an account
    #[get("/register")]
    pub async fn register(
        query: web::Query<RegisterQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = generic_http_err!(
            services::user_service::create_user(&data.pool, query.username.clone(), query.email.clone(), query.password.clone())
            .await);

        let verification = generic_http_err!(
            services::verify_service::create_verification(&data.pool, user.email.clone())
            .await);

        match emailer::send_formatted_email(
            user.email.clone(),
            "GreenPoll - Verify Account".to_string(),
            "verify".to_string(),
            [("url", FRONTEND_URL), ("verify_id", &verification.id[..])].iter().cloned().collect()
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to send verification email"))
        }?;

        Ok(success_json())
    }

    /// Logs in using email and password
    #[get("/login")]
    pub async fn login(
        query: web::Query<LoginQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let session = generic_http_err!(
            services::user_service::login(&data.pool, query.email.clone(), query.password.clone())
            .await);

        Ok(HttpResponse::Ok()
            .cookie(
                Cookie::build("session_id", session.id)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .same_site(SameSite::None)
                    .finish()
            ).json(SuccessJSON {
                success: true
            })
        )
    }

    /// Logs out
    #[get("/logout")]
    pub async fn logout(
        req: HttpRequest,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        if let Some(ref session_cookie) = req.cookie("session_id") {
            generic_http_err!(
                services::session_service::delete_session(&data.pool, session_cookie.value().to_string())
                .await);

            Ok(HttpResponse::Ok()
                .del_cookie(session_cookie)
                .json(SuccessJSON {
                    success: true
                })
            )
        } else {
            Ok(success_json())
        }
    }

    /// Logs out everywhere, removing all sessions
    #[get("/logout_everywhere")]
    pub async fn logout_everywhere(
        req: HttpRequest,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        if let Some(ref session_cookie) = req.cookie("session_id") {
            let user = get_user_by_session(&data.pool, req).await?;

            generic_http_err!(
                services::session_service::delete_user_sessions(&data.pool, user.id)
                .await);

            Ok(HttpResponse::Ok()
                .del_cookie(session_cookie)
                .json(SuccessJSON {
                    success: true
                })
            )
        } else {
            Ok(success_json())
        }
    }
}
