use actix_web::{HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON};

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct UserJSON {
    id: i32,
    email: String,
    verified: bool,
    join_time: i64,
}

pub mod user_routes {
    use super::*;

    #[get("/get_user")]
    pub async fn get_user(app_data: web::Data<Arc<Mutex<AppData>>>, query: web::Query<UserQuery>) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        let user = generic_http_err!(services::user_service::get_user(&data.pool, query.user_id).await)?;
        Ok(HttpResponse::Ok().json(UserJSON {
            id: user.id,
            email: user.email,
            verified: user.verified,
            join_time: user.join_time.timestamp()
        }))
    }
}
