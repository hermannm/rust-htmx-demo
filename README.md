# rust-htmx-demo

A simple todo app created with [HTMX](https://htmx.org/) and [Rust](https://www.rust-lang.org/),
using [axum](https://github.com/tokio-rs/axum) as the web framework,
[Maud](https://maud.lambda.xyz/) for HTML templating and [Tailwind CSS](https://tailwindcss.com/)
for styling.

## Running the app

1. Install [Rust](https://www.rust-lang.org/tools/install) and
   [Node.js](https://nodejs.org/en/download)
2. Run `npm ci` to install Tailwind
3. Run `cargo run`

For live reload in development, install [cargo-watch](https://crates.io/crates/cargo-watch) and run
`cargo watch -c -w src -x run`.
