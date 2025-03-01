use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub enum SortMethod {
    #[default]
    Relevance,
    Alphabetical,
    Created,
    Updated,
}
impl Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub enum SortOrder {
    #[default]
    Descending,
    Ascending,
}
impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub struct SortOptions {
    #[serde(default)]
    pub sort_by: SortMethod,
    #[serde(default)]
    pub order: SortOrder,
}
impl SortOptions {
    pub fn as_query(&self) -> String {
        let Self { sort_by, order } = self;
        let sort_by = sort_by.to_string();
        let order = order.to_string();
        format!("&sort_by={sort_by}&order={order}")
    }
}

fn default_page() -> u64 {
    1
}
fn default_limit() -> u64 {
    10
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub struct PagingOptions {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
}
impl PagingOptions {
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.limit
    }
    pub fn as_query(&self) -> String {
        let Self { page, limit } = self;
        format!("&page={page}&limit={limit}")
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub enum SearchMethod {
    #[default]
    Semantic,
    Metaphone,
    Soundex,
    DamerauLevenshtein,
    Levenshtein,
}
impl Display for SearchMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct SearchOptions {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub method: SearchMethod,
}
impl SearchOptions {
    pub fn as_query(&self) -> String {
        let Self { query, method } = self;
        let method = method.to_string();
        format!("&query={query}&method={method}")
    }
}

fn default_editable() -> bool {
    false
}
#[derive(Deserialize, Debug, Default, Clone, Copy)]
pub struct ViewOptions {
    #[serde(default = "default_editable")]
    pub editable: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLink {
    pub id: i64,
}

pub struct DeleteLink {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FindLink {
    pub source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryLinks {
    pub paging: PagingOptions,
    pub search: SearchOptions,
    pub sort: SortOptions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLink {
    pub source: String,
    #[serde(default)]
    pub is_alias: bool,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLink {
    pub source: String,
    #[serde(default)]
    pub is_alias: bool,
    pub target: String,
}
