use askama::Template;

use crate::model::Link;

#[derive(Template)]
#[template(path = "link.html")]
pub struct LinkTemplate {
    pub link: Link,
}

#[derive(Template)]
#[template(path = "links.html")]
pub struct LinksTemplate {
    pub links: Vec<Link>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {}
