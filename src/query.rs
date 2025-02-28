use crate::{
    model::Link,
    schema::{
        CreateLink, DeleteLink, FindLink, GetAllLinks, GetLink, SearchLinks, SearchMethod,
        UpdateLink,
    },
};

use const_format::formatcp;
use sqlx::{
    Sqlite,
    query::{Query, QueryAs, QueryScalar},
    sqlite::SqliteArguments,
};

const SEMANTIC_H: &str = r#"
    with queries as (select ? as query),
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
impl SearchLinks {
    pub fn as_semantic_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        sqlx::query_scalar::<_, i64>(formatcp!(
            r#"{SEMANTIC_H}
            select count(*) from matches;
            "#,
        ))
        .bind(&self.search.query)
    }
    fn as_semantic_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        sqlx::query_as::<_, Link>(formatcp!(
            r#"
            {SEMANTIC_H}
            select * from matches
            left join links on matches.rowid = links.id
            order by matches.distance limit ? offset ?;
            "#
        ))
        .bind(&self.search.query)
        .bind(self.filter.limit as i64)
        .bind(self.filter.offset() as i64)
    }

    pub fn as_query(&self) -> QueryAs<'_, Sqlite, Link, SqliteArguments<'_>> {
        match self.search.method {
            SearchMethod::Semantic => self.as_semantic_query(),
            SearchMethod::Metaphone => todo!(),
            SearchMethod::Soundex => todo!(),
            SearchMethod::DamerauLevenshtein => todo!(),
            SearchMethod::Levenshtein => todo!(),
        }
    }

    pub fn as_count(&self) -> QueryScalar<'_, Sqlite, i64, SqliteArguments<'_>> {
        match self.search.method {
            SearchMethod::Semantic => self.as_semantic_count(),
            SearchMethod::Metaphone => todo!(),
            SearchMethod::Soundex => todo!(),
            SearchMethod::DamerauLevenshtein => todo!(),
            SearchMethod::Levenshtein => todo!(),
        }
    }
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
