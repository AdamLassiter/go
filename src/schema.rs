use crate::model::Link;

use serde::{Deserialize, Serialize};
use sqlx::{
    Sqlite,
    query::{Query, QueryAs},
    sqlite::SqliteArguments,
};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
impl FilterOptions {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        let limit = self.limit.unwrap_or(10);
        let offset = (self.page.unwrap_or(1) - 1) * limit;

        sqlx::query_as::<_, Link>(r#"select * from links order by id limit ? offset ?"#)
            .bind(limit as i32)
            .bind(offset as i32)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLink {
    pub id: i64,
}
impl GetLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(r#"select * from links where id = ?"#).bind(self.id)
    }
}

pub struct DeleteLink {
    pub id: i64,
}
impl DeleteLink {
    pub fn as_query(&self) -> Query<'_, Sqlite, SqliteArguments<'_>> {
        sqlx::query(r#"delete from links where id = ?"#).bind(self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FindLink {
    pub source: String,
}
impl FindLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(r#"select * from links where source = ?"#).bind(&self.source)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLink {
    pub source: String,
    #[serde(default)]
    pub is_alias: bool,
    pub target: String,
}
impl CreateLink {
    pub fn as_query(&self) -> Query<'_, Sqlite, SqliteArguments<'_>> {
        sqlx::query(r#"insert into links (source, is_alias, target) values (?, ?, ?)"#)
            .bind(&self.source)
            .bind(self.is_alias)
            .bind(&self.target)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLink {
    pub source: String,
    #[serde(default)]
    pub is_alias: bool,
    pub target: String,
}
impl UpdateLink {
    pub fn as_query(&self, id: i64) -> Query<'_, Sqlite, SqliteArguments<'_>> {
        sqlx::query(r#"update links where id = ? set source = ?, is_alias = ?, target = ?"#)
            .bind(id)
            .bind(&self.source)
            .bind(self.is_alias)
            .bind(&self.target)
    }
}
