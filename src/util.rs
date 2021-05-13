use serde::{Serialize, Deserialize};

pub type DBPool = sqlx::Pool<sqlx::Postgres>;

pub struct AppData {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorJSON {
    pub error: String,
}

#[macro_export]
macro_rules! generic_http_err {
    ( $x:expr ) => {
        match $x {
            Ok(val) => Ok(val),
            Err(e) => Err(HttpResponse::Ok().json(ErrorJSON {
                error: format!("{}", e)
            })),
        }?
    };
}

#[macro_export]
macro_rules! generic_service_err {
    ( $x:expr, $err:literal ) => {
        match $x {
            Ok(res) => Ok(res),
            Err(_) => Err(Error::new(ErrorKind::Other, $err))
        }?
    };
}
