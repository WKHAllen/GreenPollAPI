use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, error_json, get_user_by_session};
use crate::routes::{PollOptionJSON, PollVoteJSON};

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
pub struct GetPollOptionsQuery {
    poll_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GetPollVotesQuery {
    poll_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct SetTitleQuery {
    poll_id: i32,
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetDescriptionQuery {
    poll_id: i32,
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePollQuery {
    poll_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct PollJSON {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub create_time: i64,
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

        let poll = generic_http_err!(
            services::poll_service::create_poll(&data.pool, user.id, query.title.clone(), query.description.clone())
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

    #[get("/get_poll_options")]
    pub async fn get_poll_options(
        query: web::Query<GetPollOptionsQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll_options = generic_http_err!(
            services::poll_service::get_poll_options(&data.pool, query.poll_id)
            .await);

        let options: Vec<PollOptionJSON> = poll_options.iter().map(|option| PollOptionJSON {
            id: option.id,
            poll_id: option.poll_id,
            value: option.value.clone()
        }).collect();

        Ok(HttpResponse::Ok().json(options))
    }

    #[get("/get_poll_votes")]
    pub async fn get_poll_votes(
        query: web::Query<GetPollVotesQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll_votes = generic_http_err!(
            services::poll_service::get_poll_votes(&data.pool, query.poll_id)
            .await);

        let votes: Vec<PollVoteJSON> = poll_votes.iter().map(|vote| PollVoteJSON {
            id: vote.id,
            user_id: vote.user_id,
            poll_id: vote.poll_id,
            poll_option_id: vote.poll_option_id,
            vote_time: vote.vote_time.timestamp()
        }).collect();

        Ok(HttpResponse::Ok().json(votes))
    }

    #[get("/set_poll_title")]
    pub async fn set_poll_title(
        req: HttpRequest,
        query: web::Query<SetTitleQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, query.poll_id)
            .await);

        if user.id == poll.user_id {
            generic_http_err!(
                services::poll_service::set_title(&data.pool, query.poll_id, query.title.clone())
                .await);

            Ok(success_json())
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }

    #[get("/set_poll_description")]
    pub async fn set_poll_description(
        req: HttpRequest,
        query: web::Query<SetDescriptionQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, query.poll_id)
            .await);

        if user.id == poll.user_id {
            generic_http_err!(
                services::poll_service::set_description(&data.pool, query.poll_id, query.title.clone())
                .await);

            Ok(success_json())
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }

    #[get("/delete_poll")]
    pub async fn delete_poll(
        req: HttpRequest,
        query: web::Query<DeletePollQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;
        let poll = generic_http_err!(
            services::poll_service::get_poll(&data.pool, query.poll_id)
            .await);

        if user.id == poll.user_id {
            generic_http_err!(
                services::poll_service::delete_poll(&data.pool, query.poll_id)
                .await);

            Ok(success_json())
        } else {
            Ok(error_json("You do not have permission to edit this poll"))
        }
    }
}
