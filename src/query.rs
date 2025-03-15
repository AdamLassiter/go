use crate::{
    model::Link,
    schema::{CreateLink, DeleteLink, FindLink, GetLink, QueryLinks, SearchMethod, UpdateLink},
};

use sqlx::{
    Sqlite,
    query::{Query, QueryAs, QueryScalar},
    sqlite::SqliteArguments,
};
use static_str_ops::static_format;

const SEMANTIC_H: &str = r#"with
    queries as (select ? as query),
    matches as (
        select rowid, distance
        from vec_links, queries
        where vec_source match lembed(query)
        and k = 100
        --     union
        -- select rowid, distance
        -- from vec_links, queries
        -- where vec_description match lembed(query)
        -- and k = 100
    )
"#;
impl QueryLinks {
    pub fn as_semantic_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        sqlx::query_scalar::<_, i64>(static_format!(
            r#"{SEMANTIC_H}
            select count(*) from matches;
            "#,
        ))
        .bind(&self.search.query)
    }

    fn as_semantic_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(static_format!(
            r#"
            {SEMANTIC_H}
            select * from matches
            left join links on matches.rowid = links.id
            order by matches.distance asc
            limit ? offset ?;
            "#
        ))
        .bind(&self.search.query)
        .bind(self.paging.limit as i64)
        .bind(self.paging.offset() as i64)
    }

    fn as_damlev_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        sqlx::query_scalar(static_format!(
            r#"
            select count(*) from links
            "#,
        ))
        .bind(&self.search.query)
    }

    fn as_damlev_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(static_format!(
            r#"
            select * from links
            order by fuzzy_damlev(links.source, ?) asc
            limit ? offset ?;
            "#,
        ))
        .bind(&self.search.query)
        .bind(self.paging.limit as i64)
        .bind(self.paging.offset() as i64)
    }

    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        match self.search.method {
            SearchMethod::Semantic => self.as_semantic_query(),
            SearchMethod::DamerauLevenshtein => self.as_damlev_query(),
        }
    }

    pub fn as_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        match self.search.method {
            SearchMethod::Semantic => self.as_semantic_count(),
            SearchMethod::DamerauLevenshtein => self.as_damlev_count(),
        }
    }
}

impl GetLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(r#"select * from links where id = ?"#).bind(self.id)
    }
}

impl FindLink {
    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(r#"select * from links where source = ?"#).bind(&self.source)
    }
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

impl DeleteLink {
    pub fn as_query(&self) -> Query<'_, Sqlite, SqliteArguments<'_>> {
        sqlx::query(r#"delete from links where id = ?"#).bind(self.id)
    }
}
