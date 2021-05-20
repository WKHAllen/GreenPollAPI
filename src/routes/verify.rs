use actix_web::{HttpResponse, Result, web, get};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use crate::{services, generic_http_err};
use crate::util::{AppData, ErrorJSON, success_json};

/// Query parameters for account verification
#[derive(Serialize, Deserialize)]
pub struct VerifyAccountQuery {
    verify_id: String,
}

/// The verification routes
pub mod verify_routes {
    use super::*;

    /// Verifies a user's account
    #[get("/verify_account")]
    pub async fn verify_account(
        query: web::Query<VerifyAccountQuery>,
        app_data: web::Data<Arc<Mutex<AppData>>>
    ) -> Result<HttpResponse> {
        let data = app_data.lock().unwrap();

        generic_http_err!(
            services::verify_service::verify_user(&data.pool, query.verify_id.clone())
            .await);

        Ok(success_json())
    }
}
