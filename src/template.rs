use askama::Template;

use crate::model::{Link, Paging};

#[derive(Template)]
#[template(path = "links/view.html")]
pub struct ViewTemplate {
    pub link: Link,
}

#[derive(Template)]
#[template(path = "links/list.html")]
pub struct ListTemplate {
    pub links: Vec<Link>,
    pub paging: Paging,
}

#[derive(Template)]
#[template(path = "pages/links.html")]
pub struct LinksTemplate {}

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorTemplate {}
