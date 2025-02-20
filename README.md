# Go!

A crowdsourced search service, built with Rust and SQLite.

## Getting Started

### Rust

Go! is built using Rust, you can install it with `rustup`:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Some sane defaults might be as follows:
```sh
rustup default 1.85.0
rustup component add rust-analyzer
```

### SQLx
See [schema.rs](src/schema.rs)

Go! uses [SQLx](https://docs.rs/sqlx/latest/sqlx/) for managing database migrations, you can install it with `cargo`:
```sh
cargo install sqlx-cli
```

While SQLx _can_ provide compile-time checks of SQL queries, this feature is only available through the provided macros; these suffer some ergonomic downsides which the gains do not justify.

### Askama
See [template.rs](src/template.rs)

Go! uses server-side template rendering from [Askama](https://docs.rs/askama/latest/askama/).

Jinja-like syntax is used to build HTML templates:
```jinja
<div id="paged-links">
    <div id="links-content">
        {% for link in links %}
        {%- include "links/view.html" -%}
        {% endfor %}
    </div>
    <div id="paging">
        {%- include "utils/paging.html" -%}
    </div>
</div>
```

These templates are then bound to Rust structs:
```rust
#[derive(Template)]
#[template(path = "paged-links.html")]
pub struct PagedLinksTemplate {
    pub links: Vec<Link>,
    pub paging: Paging,
}
```

Askama provides type-checking of templates at compile-time.

### HTMX
See [templates](templates)

Go! adds interactivity to frontend pages using [HTMX](https://htmx.org/docs/).

This is generally achieved through HTTP <verb> requests which recieve HTML fragment responses, which are swapped in and out of the DOM.

# Some Ideas

* links/
  * search
  * view
  * list
  * edit
  * create
* pages/
  * popular
  * recent
  * mine
* utils/
  * paging
