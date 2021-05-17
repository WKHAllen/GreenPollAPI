use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, error_json, get_user_by_session};
use crate::routes::PollJSON;

#[derive(Serialize, Deserialize)]
pub struct CreatePollOptionQuery {
    poll_id: i32,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPollOptionQuery {
    poll_option_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SetPollOptionValueQuery {
    poll_option_id: i32,
    new_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPollOptionPollQuery {
    poll_option_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePollOptionQuery {
    poll_option_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PollOptionJSON {
    id: i32,
    poll_id: i32,
    value: String,
}

pub mod poll_option_routes {
    use super::*;

    #[get("/create_poll_option")]
    pub async fn create_poll_option(
        req: HttpRequest,
        query: web::Query<CreatePollOptionQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, query.poll_id)
            .await);

        if user.id == poll.user_id {
            let poll_option = generic_http_err!(
                services::poll_option_service::create_poll_option(&data.pool, query.poll_id, query.value.clone())
                .await);

            Ok(HttpResponse::Ok().json(PollOptionJSON {
                id: poll_option.id,
                poll_id: poll_option.poll_id,
                value: poll_option.value
            }))
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }

    #[get("/get_poll_option_info")]
    pub async fn get_poll_option_info(
        query: web::Query<GetPollOptionQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll_option = generic_http_err!(
            services::poll_option_service::get_poll_option(&data.pool, query.poll_option_id)
            .await);

        Ok(HttpResponse::Ok().json(PollOptionJSON {
            id: poll_option.id,
            poll_id: poll_option.poll_id,
            value: poll_option.value
        }))
    }

    #[get("/set_poll_option_value")]
    pub async fn set_poll_option_value(
        req: HttpRequest,
        query: web::Query<SetPollOptionValueQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_option_service::get_poll_option_poll(&data.pool, query.poll_option_id)
            .await);

        if user.id == poll.user_id {
            generic_http_err!(
                services::poll_option_service::set_poll_option_value(&data.pool, query.poll_option_id, query.new_value.clone())
                .await);

            Ok(success_json())
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }

    #[get("/get_poll_option_poll")]
    pub async fn get_poll_option_poll(
        query: web::Query<GetPollOptionPollQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll = generic_http_err!(
            services::poll_option_service::get_poll_option_poll(&data.pool, query.poll_option_id)
            .await);

        Ok(HttpResponse::Ok().json(PollJSON {
            id: poll.id,
            user_id: poll.user_id,
            title: poll.title,
            description: poll.description,
            create_time: poll.create_time.timestamp()
        }))
    }

    #[get("/delete_poll_option")]
    pub async fn delete_poll_option(
        req: HttpRequest,
        query: web::Query<DeletePollOptionQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_option_service::get_poll_option_poll(&data.pool, query.poll_option_id)
            .await);

        if user.id == poll.user_id {
            generic_http_err!(
                services::poll_option_service::delete_poll_option(&data.pool, query.poll_option_id)
                .await);

            Ok(success_json())
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }
}
