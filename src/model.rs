use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Link {
    pub id: i64,

    // created_by: String,
    pub created_at: NaiveDateTime,
    // modified_by: String,
    pub modified_at: NaiveDateTime,

    pub source: String,
    pub is_alias: bool,
    pub target: String,
    pub description: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Default)]
pub struct Paging {
    pub page: u64,
    pub limit: u64,
    pub last: u64,

    pub source: String,
    pub target: String,
}
