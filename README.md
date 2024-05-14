# rust-htmx-demo

A simple todo app created with [HTMX](https://htmx.org/) and [Rust](https://www.rust-lang.org/),
using [axum](https://github.com/tokio-rs/axum) as the web framework,
[Maud](https://maud.lambda.xyz/) for HTML templating and [Tailwind CSS](https://tailwindcss.com/)
for styling.

## Running the app

1. Install [Rust](https://www.rust-lang.org/tools/install) and
   [Node.js](https://nodejs.org/en/download)
2. Run `npm ci` (required for Tailwind CSS generation and TypeScript compilation)
3. Copy `.env.example` to a new file named `.env` in the root of the repository
4. Run `cargo run` (this runs Tailwind and TypeScript as part of the build process)

For live reload in development, install [cargo-watch](https://crates.io/crates/cargo-watch) and run
`cargo watch -c -w src -x run`.
