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

Go! uses [SQLx](https://docs.rs/sqlx/latest/sqlx/) for managing database migrations, you can install it with `cargo`:
```sh
cargo install sqlx-cli
```

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
