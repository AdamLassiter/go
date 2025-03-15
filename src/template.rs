use askama::Template;

use crate::{
    model::{Link, Paging},
    schema::{CreateLink, PagingOptions, SearchOptions},
};

#[derive(Template)]
#[template(path = "links/view.html")]
pub struct CreateTemplate {
    pub link: Link,
}

#[derive(Template)]
#[template(path = "links/view.html")]
pub struct ViewTemplate {
    pub link: Link,
}

#[derive(Template)]
#[template(path = "links/edit.html")]
pub struct EditTemplate {
    pub link: Link,
}

#[derive(Template)]
#[template(path = "links/list.html")]
pub struct ListTemplate {
    pub new: Option<CreateLink>,
    pub links: Vec<Link>,
    pub paging: Paging,
}

#[derive(Template)]
#[template(path = "links/search.html")]
pub struct SearchTemplate {
    pub search: SearchOptions,
}

#[derive(Template)]
#[template(path = "pages/links.html")]
pub struct LinksTemplate {
    pub paging: PagingOptions,
    pub search: SearchOptions,
}

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorTemplate {}
