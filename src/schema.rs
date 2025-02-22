use crate::model::{Link, Paging};

use serde::{Deserialize, Serialize};
use sqlx::{
    Sqlite,
    query::{Query, QueryAs, QueryScalar},
    sqlite::SqliteArguments,
};

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

#[derive(Deserialize, Debug, Default, Clone, Copy)]
pub struct GetAllLinks {
    pub filter: FilterOptions,
}
impl GetAllLinks {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(
            r#"select * from links order by modified_at desc limit ? offset ?"#,
        )
        .bind(self.filter.limit as i64)
        .bind(self.filter.offset() as i64)
    }

    pub fn as_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        sqlx::query_scalar(r#"select count(*) from links"#)
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
    pub query: String,
}
impl FindLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(r#"select * from links where source = ?"#).bind(&self.query)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchLink {
    pub query: String,
    pub filter: FilterOptions,
}
impl SearchLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(
            r#"
        .param set :query ?
        with matches as ((
            select id, distance from vec_links
            where vec_alias match lembed('minilm', :query)
            union
            select id, distance from vec_links
            where vec_description match lembed('minilm', :query)
        ) order by distance limit )
        select * from matches where source = ?
        "#,
        )
        .bind(&self.query)
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
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(
            r#"insert into links (source, is_alias, target) values (?, ?, ?) returning *"#,
        )
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
    pub fn as_query(&self, id: i64) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as(
            r#"update links set source = ?, is_alias = ?, target = ? where id = ? returning *"#,
        )
        .bind(&self.source)
        .bind(self.is_alias)
        .bind(&self.target)
        .bind(id)
    }
}
