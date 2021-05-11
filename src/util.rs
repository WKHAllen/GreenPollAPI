use serde::{Serialize, Deserialize};

pub type DBPool = sqlx::Pool<sqlx::Postgres>;

#[derive(Serialize, Deserialize)]
pub struct ErrorJSON {
    pub error: String,
}

pub struct AppData {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}
