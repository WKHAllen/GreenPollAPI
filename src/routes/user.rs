use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, get_user_by_session};

/// Query parameters for setting a user's username
#[derive(Serialize, Deserialize)]
pub struct SetUsernameQuery {
    new_username: String,
}

/// Query parameters for setting a user's password
#[derive(Serialize, Deserialize)]
pub struct SetPasswordQuery {
    new_password: String,
}

/// JSON representation of a user
#[derive(Serialize, Deserialize)]
pub struct UserJSON {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub join_time: i64,
}

/// The user routes
pub mod user_routes {
    use super::*;

    /// Returns a user's details
    #[get("/get_user_info")]
    pub async fn get_user_info(
        req: HttpRequest,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        Ok(HttpResponse::Ok().json(UserJSON {
            id: user.id,
            username: user.username,
            email: user.email,
            join_time: user.join_time.timestamp()
        }))
    }

    /// Sets a user's username
    #[get("/set_username")]
    pub async fn set_username(
        req: HttpRequest,
        query: web::Query<SetUsernameQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        generic_http_err!(
            services::user_service::set_username(&data.pool, user.id, query.new_username.clone())
            .await);

        Ok(success_json())
    }

    /// Sets a user's password
    #[get("/set_password")]
    pub async fn set_password(
        req: HttpRequest,
        query: web::Query<SetPasswordQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        generic_http_err!(
            services::user_service::set_username(&data.pool, user.id, query.new_password.clone())
            .await);

        Ok(success_json())
    }
}
