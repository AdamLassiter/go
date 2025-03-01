use sqlx::Error;

use crate::{
    AppState,
    model::Link,
    schema::{CreateLink, DeleteLink, FindLink, GetLink, QueryLinks, UpdateLink},
};

pub async fn query_links(
    app_state: &AppState,
    query: &QueryLinks,
) -> Result<(Vec<Link>, u64), Error> {
    println!(
        "💽 Search for '{}' with strategy '{}', page '{}' size '{}', sorted by '{}' '{}'",
        query.search.query,
        query.search.method,
        query.paging.page,
        query.paging.limit,
        query.sort.sort_by,
        query.sort.order
    );
    let links = query.as_query().fetch_all(&app_state.db).await?;
    let count = query.as_count().fetch_one(&app_state.db).await?;
    let last = (count as u64).div_ceil(query.paging.limit);

    Ok((links, last))
}

pub async fn create_link(app_state: &AppState, create: &CreateLink) -> Result<Link, Error> {
    println!("💽 Create new '{}'", create.source);
    let link = create.as_query().fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn find_link(app_state: &AppState, find: &FindLink) -> Result<Option<Link>, Error> {
    println!("💽 Find '{}'", find.source);
    let link = find.as_query().fetch_optional(&app_state.db).await?;

    Ok(link)
}

pub async fn get_link(app_state: &AppState, get: &GetLink) -> Result<Link, Error> {
    println!("💽 Get '{}'", get.id);
    let link = get.as_query().fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn edit_link(
    app_state: &AppState,
    get: &GetLink,
    update: &UpdateLink,
) -> Result<Link, Error> {
    println!("💽 Edit '{}'", get.id);
    get_link(app_state, get).await?;
    let link = update.as_query(get.id).fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn delete_link(app_state: &AppState, delete: &DeleteLink) -> Result<(), Error> {
    println!("💽 Delete '{}'", delete.id);
    let query_result = delete.as_query().execute(&app_state.db).await?;

    if query_result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    Ok(())
}
