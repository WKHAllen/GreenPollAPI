use actix_web::{HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::services;
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

#[get("/get_user")]
pub async fn get_user(app_data: web::Data<Arc<Mutex<AppData>>>, query: web::Query<UserQuery>) -> Result<HttpResponse> {
    let data = app_data.lock().unwrap();

    match services::user_service::get_user(&data.pool, query.user_id).await {
        Ok(user) => Ok(HttpResponse::Ok().json(UserJSON {
            id: user.id,
            email: user.email,
            verified: user.verified,
            join_time: user.join_time.timestamp()
        })),
        Err(e) => Ok(HttpResponse::Ok().json(ErrorJSON {
            error: format!("{}", e)
        })),
    }
}
