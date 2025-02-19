use sqlx::Error;

use crate::{
    AppState,
    model::Link,
    schema::{CreateLink, DeleteLink, FilterOptions, FindLink, GetLink, UpdateLink},
};

pub async fn get_links(app_state: &AppState, opts: &FilterOptions) -> Result<(Vec<Link>, usize), Error> {
    let links = opts.as_query().fetch_all(&app_state.db).await?;
    let count = opts.as_count().fetch_one(&app_state.db).await?;
    let last = (count as usize).div_ceil(opts.limit);

    Ok((links, last))
}

pub async fn create_link(app_state: &AppState, create: &CreateLink) -> Result<Link, Error> {
    let link = create.as_query().fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn find_link(app_state: &AppState, find: &FindLink) -> Result<Link, Error> {
    let link = find.as_query().fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn get_link(app_state: &AppState, get: &GetLink) -> Result<Link, Error> {
    let link = get.as_query().fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn edit_link(
    app_state: &AppState,
    get: &GetLink,
    update: &UpdateLink,
) -> Result<Link, Error> {
    // Check exists
    get_link(app_state, get).await?;

    let link = update.as_query(get.id).fetch_one(&app_state.db).await?;

    Ok(link)
}

pub async fn delete_link(app_state: &AppState, delete: &DeleteLink) -> Result<(), Error> {
    let query_result = delete.as_query().execute(&app_state.db).await?;

    if query_result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    Ok(())
}
