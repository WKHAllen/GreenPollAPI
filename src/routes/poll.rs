use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, get_user_by_session};

#[derive(Serialize, Deserialize)]
pub struct CreatePollQuery {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPollQuery {
    poll_id: i32,
}

#[derive(Serialize, Deserialize)]
struct PollJSON {
    id: i32,
    user_id: i32,
    title: String,
    description: String,
    create_time: i64,
}

pub mod poll_routes {
    use super::*;

    #[get("/create_poll")]
    pub async fn create_poll(
        req: HttpRequest,
        query: web::Query<CreatePollQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        let poll_id = generic_http_err!(
            services::poll_service::create_poll(&data.pool, user.id, query.title.clone(), query.description.clone())
            .await);

        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, poll_id)
            .await);

        Ok(HttpResponse::Ok().json(PollJSON {
            id: poll.id,
            user_id: poll.user_id,
            title: poll.title,
            description: poll.description,
            create_time: poll.create_time.timestamp()
        }))
    }

    #[get("/get_poll_info")]
    pub async fn get_poll_info(
        query: web::Query<GetPollQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, query.poll_id)
            .await);

        Ok(HttpResponse::Ok().json(PollJSON {
            id: poll.id,
            user_id: poll.user_id,
            title: poll.title,
            description: poll.description,
            create_time: poll.create_time.timestamp()
        }))
    }
}