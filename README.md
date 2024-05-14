# rust-htmx-demo

A simple todo app created with [HTMX](https://htmx.org/) and [Rust](https://www.rust-lang.org/),
using [axum](https://github.com/tokio-rs/axum) as the web framework,
[Maud](https://maud.lambda.xyz/) for HTML templating, [SQLx](https://github.com/launchbadge/sqlx)
for database queries and [Tailwind CSS](https://tailwindcss.com/) for styling.

## Running the app

1. Install [Rust](https://www.rust-lang.org/tools/install),
   [Node.js](https://nodejs.org/en/download) and
   [Docker Compose](https://docs.docker.com/compose/install/)
2. Run `npm ci` (required for Tailwind CSS generation and TypeScript compilation)
3. Install the [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) for
   compile-time verification of SQL queries:
   `cargo install sqlx-cli --no-default-features --features postgres,rustls`
4. Copy `.env.example` to a file named `.env` in the repo root

   - This is required for SQLx compile-time verification to work, as it needs the `DATABASE_URL`
     environment variable in a `.env` file

5. Run `docker compose up` to start Postgres
6. Run `sqlx migrate run` to create our database tables
7. Run `cargo run` (this runs Tailwind and TypeScript as part of the build process)

For live reload in development, install [cargo-watch](https://crates.io/crates/cargo-watch) and run
`cargo watch -c -w src -x run`.
