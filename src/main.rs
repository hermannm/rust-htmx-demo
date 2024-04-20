use anyhow::{Context, Result};
use app::App;
use config::{Config, Environment};
use repository::TodoRepository;
use tracing::Level;

mod app;
mod config;
mod repository;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = Config::load().context("Failed to load config")?;

    let todo_repo = TodoRepository::new();
    if let Environment::Dev = config.environment {
        todo_repo
            .add_examples()
            .context("Failed to add example todos")?;
    }

    let app = App::new(todo_repo);
    app.serve_api(&config).await?;

    Ok(())
}
