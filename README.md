# Go!

A crowdsourced search service, built with Rust and SQLite.

## Getting Started

### Rust
See [main.rs](src/main.rs) and [Cargo.toml](Cargo.toml)

Go! is built using Rust, you can install it - and other toolchain components - with [rustup](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Some sane defaults might be as follows:
```sh
rustup default 1.85.0
rustup component add rust-analyzer
```

Using Cargo as the de-facto Rust package-manager and build-tool, running the application is then simply:
```sh
cargo run
```


### SQLx
See [schema.rs](src/schema.rs) and [model.rs](src/model.rs)

Go! uses [SQLx](https://docs.rs/sqlx/latest/sqlx/) for managing database migrations, you can install it with `cargo`:
```sh
cargo install sqlx-cli
```

After installing, migrations can be added to [migrations](migrations), and managed using the `sqlx` command:
```sh
sqlx db create
sqlx migrate run
    <!> oh no <!>
sqlx migrate revert
```

While SQLx _can_ provide compile-time checks of SQL queries, this feature is only available through the provided macros; these suffer some ergonomic downsides which the gains do not justify.

### SQLite
See [schema.rs](src/schema.rs)

Go! specifically uses [SQLite](https://www.sqlite.org/docs.html) as the SQL database engine of choice - it's quick enough, runs locally to minimise required infrastructure, and allows backups to be as trivial as a single file copy.

### Askama
See [template.rs](src/template.rs) and [templates](templates)

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
See [templates](templates) and [route.rs](src/route.rs)

Go! adds interactivity to frontend pages using [HTMX](https://htmx.org/docs/).

This is generally achieved through HTTP <verb> requests which recieve HTML fragment responses, which are swapped in and out of the DOM.

### Bootstrap

Go! uses [Bootstrap](https://getbootstrap.com/docs/5.3/getting-started/introduction/) as a component library and UI toolkit, allowing components to be styled easily and consistently without resorting to CSS fragments.

Bootstrap provides a vast array of CSS classes that can be combined to achieve most common web-app usecases, e.g. a radio button group with:
```html
<div class="btn-group" role="group" aria-label="Basic radio toggle button group">
  <input type="radio" class="btn-check" name="btnradio" id="btnradio1" autocomplete="off" checked>
  <label class="btn btn-outline-primary" for="btnradio1">Radio 1</label>

  <input type="radio" class="btn-check" name="btnradio" id="btnradio2" autocomplete="off">
  <label class="btn btn-outline-success" for="btnradio2">Radio 2</label>

  <input type="radio" class="btn-check" name="btnradio" id="btnradio3" autocomplete="off">
  <label class="btn btn-outline-danger" for="btnradio3">Radio 3</label>
</div>
```

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
