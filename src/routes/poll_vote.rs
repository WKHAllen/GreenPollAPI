use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, get_user_by_session};
use crate::routes::PollJSON;

/// Query parameters for voting on a poll
#[derive(Serialize, Deserialize)]
pub struct PollVoteQuery {
    poll_option_id: i32,
}

/// Query parameters for removing a vote from a poll
#[derive(Serialize, Deserialize)]
pub struct PollUnvoteQuery {
    poll_id: i32,
}

/// Query parameters for getting the poll associated with a poll vote
#[derive(Serialize, Deserialize)]
pub struct GetPollVotePollQuery {
    poll_vote_id: i32,
}

/// JSON representation of a poll vote
#[derive(Serialize, Deserialize)]
pub struct PollVoteJSON {
    pub id: i32,
    pub user_id: i32,
    pub poll_id: i32,
    pub poll_option_id: i32,
    pub vote_time: i64,
}

/// The poll vote routes
pub mod poll_vote_routes {
    use super::*;

    /// Votes on a poll and returns the resulting record
    #[get("/poll_vote")]
    pub async fn poll_vote(
        req: HttpRequest,
        query: web::Query<PollVoteQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        let poll_vote = generic_http_err!(
            services::poll_vote_service::vote(&data.pool, user.id, query.poll_option_id)
            .await);

        Ok(HttpResponse::Ok().json(PollVoteJSON {
            id: poll_vote.id,
            user_id: poll_vote.user_id,
            poll_id: poll_vote.poll_id,
            poll_option_id: poll_vote.poll_option_id,
            vote_time: poll_vote.vote_time.timestamp()
        }))
    }

    /// Removes a vote from a poll
    #[get("/poll_unvote")]
    pub async fn poll_unvote(
        req: HttpRequest,
        query: web::Query<PollUnvoteQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = get_user_by_session(&data.pool, req).await?;

        generic_http_err!(
            services::poll_vote_service::unvote(&data.pool, user.id, query.poll_id)
            .await);

        Ok(success_json())
    }

    /// Returns the poll associated with the poll vote
    #[get("/get_poll_vote_poll")]
    pub async fn get_poll_vote_poll(
        query: web::Query<GetPollVotePollQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let poll = generic_http_err!(
            services::poll_vote_service::get_poll_vote_poll(&data.pool, query.poll_vote_id)
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
