
use sqlx::Error;

use crate::{
    AppState,
    model::Link,
    schema::{CreateLink, DeleteLink, FilterOptions, FindLink, GetLink, UpdateLink},
};

pub async fn get_links(app_state: &AppState, opts: &FilterOptions) -> Result<Vec<Link>, Error> {
    let links = opts.as_query().fetch_all(&app_state.db).await?;

    Ok(links)
}

pub async fn create_link(app_state: &AppState, create: &CreateLink) -> Result<Link, Error> {
    create.as_query().execute(&app_state.db).await?;

    let find = FindLink {
        source: create.source.clone(),
    };
    find_link(app_state, &find).await
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

    let update_result = update.as_query(get.id).execute(&app_state.db).await?;

    if update_result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    get_link(app_state, get).await
}

pub async fn delete_link(app_state: &AppState, delete: &DeleteLink) -> Result<(), Error> {
    let query_result = delete.as_query().execute(&app_state.db).await?;

    if query_result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    Ok(())
}
