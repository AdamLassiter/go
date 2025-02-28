use crate::model::Paging;

use serde::{Deserialize, Serialize};

fn default_page() -> u64 {
    1
}
fn default_limit() -> u64 {
    10
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub struct FilterOptions {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_limit")]
    pub limit: u64,
}
impl FilterOptions {
    pub fn into_paging(self, last: u64, source: &str, target: &str) -> Paging {
        let page = self.page;
        let limit = self.limit;
        Paging {
            page,
            limit,
            last,
            source: source.to_string(),
            target: target.to_string(),
        }
    }
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.limit
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
impl SearchMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::Metaphone => "metaphone",
            Self::Soundex => "soundex",
            Self::DamerauLevenshtein => "damerau",
            Self::Levenshtein => "levenshtein",
        }
    }
    pub fn try_from_str(string: &str) -> Option<Self> {
        match string {
            "semantic" => Some(Self::Semantic),
            "metaphone" => Some(Self::Metaphone),
            "soundex" => Some(Self::Soundex),
            "damerau" => Some(Self::DamerauLevenshtein),
            "levenshtein" => Some(Self::Levenshtein),
            _ => None
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct SearchOptions {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub method: SearchMethod,
}

#[derive(Deserialize, Debug, Default, Clone, Copy)]
pub struct GetAllLinks {
    pub filter: FilterOptions,
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
pub struct SearchLinks {
    pub filter: FilterOptions,
    pub search: SearchOptions,
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
