use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, error_json, get_user_by_session};
use crate::routes::PollJSON;

/// Query parameters for creating a poll option
#[derive(Serialize, Deserialize)]
pub struct CreatePollOptionQuery {
    poll_id: i32,
    value: String,
}

/// Query parameters for getting a poll option
#[derive(Serialize, Deserialize)]
pub struct GetPollOptionQuery {
    poll_option_id: i32,
}

/// Query parameters for setting a poll option's text representation
#[derive(Serialize, Deserialize)]
pub struct SetPollOptionValueQuery {
    poll_option_id: i32,
    new_value: String,
}

/// Query parameters for getting the poll associated with a poll option
#[derive(Serialize, Deserialize)]
pub struct GetPollOptionPollQuery {
    poll_option_id: i32,
}

/// Query parameters for deleting a poll option
#[derive(Serialize, Deserialize)]
pub struct DeletePollOptionQuery {
    poll_option_id: i32,
}

/// JSON representation of a poll option
#[derive(Serialize, Deserialize)]
pub struct PollOptionJSON {
    pub id: i32,
    pub poll_id: i32,
    pub value: String,
}

/// The poll option routes
pub mod poll_option_routes {
    use super::*;

    /// Creates a poll option and returns the resulting record
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

    /// Returns the poll option details
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

    /// Sets the text representation of a poll option
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

    /// Returns the poll associated with a poll option
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

    /// Deletes a poll option
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
