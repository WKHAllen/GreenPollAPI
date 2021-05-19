use actix_web::{HttpRequest, HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use std::io::{Error, ErrorKind};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json, error_json, get_user_by_session};
use crate::routes::{PollOptionJSON, PollVoteJSON};
use crate::emailer;

#[derive(Serialize, Deserialize)]
pub struct RegisterQuery {
    username: String,
    email: String,
    password: String,
}

pub mod login_register_routes {
    use super::*;

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
            [("verify_id", &verification.id[..])].iter().cloned().collect()
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(ErrorKind::Other, "Failed to send verification email"))
        }?;

        Ok(success_json())
    }
}
