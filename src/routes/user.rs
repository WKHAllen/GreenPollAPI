use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::routes::PollJSON;
use crate::util::{AppData, ErrorJSON, success_json, get_user_by_session};

/// Query parameters for getting a specific user's info
#[derive(Serialize, Deserialize)]
pub struct GetSpecificUserQuery {
    user_id: i32,
}

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

/// JSON representation of a user as seen by another user
#[derive(Serialize, Deserialize)]
pub struct SpecificUserJSON {
    pub id: i32,
    pub username: String,
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

    /// Returns a specified user's details
    #[get("/get_specific_user_info")]
    pub async fn get_specific_user_info(
        query: web::Query<GetSpecificUserQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = generic_http_err!(
            services::user_service::get_user(&data.pool, query.user_id)
            .await);

        Ok(HttpResponse::Ok().json(SpecificUserJSON {
            id: user.id,
            username: user.username,
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
            services::user_service::set_password(&data.pool, user.id, query.new_password.clone())
            .await);

        Ok(success_json())
    }

    /// Gets a user's polls
    #[get("/get_user_polls")]
    pub async fn get_user_polls(
        req: HttpRequest,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        let user_polls = generic_http_err!(
            services::user_service::get_user_polls(&data.pool, user.id)
            .await);

        let polls: Vec<PollJSON> = user_polls.iter().map(|poll| PollJSON {
            id: poll.id,
            user_id: poll.user_id,
            title: poll.title.clone(),
            description: poll.description.clone(),
            create_time: poll.create_time.timestamp()
        }).collect();

        Ok(HttpResponse::Ok().json(polls))
    }
}
