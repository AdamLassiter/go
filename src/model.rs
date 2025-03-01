use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::schema::{PagingOptions, SearchOptions, SortOptions};

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
impl Paging {
    pub fn new(
        paging: &PagingOptions,
        search: &SearchOptions,
        sort: &SortOptions,
        last: u64,
        source: &str,
        target: &str,
    ) -> Self {
        let PagingOptions { page, limit } = *paging;
        let source = format!(
            "{source}?{}{}",
            search.as_query(),
            sort.as_query()
        );
        Self {
            page,
            limit,
            last,
            source,
            target: target.to_string(),
        }
    }
}
