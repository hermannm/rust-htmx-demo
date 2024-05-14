use anyhow::{Context, Result};
use app::App;
use config::{Config, Environment};
use db::TodoDatabase;
use tracing::Level;

mod app;
mod config;
mod db;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let config = Config::load().context("Failed to load config")?;

    let todo_repo = TodoDatabase::new(&config).await?;
    if let Environment::Dev = config.environment {
        todo_repo
            .add_examples()
            .await
            .context("Failed to add example todos")?;
    }

    let app = App::new(todo_repo);
    app.serve_api(&config).await?;

    Ok(())
}
