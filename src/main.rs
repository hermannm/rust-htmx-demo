use anyhow::{Context, Result};
use app::App;
use config::{Config, Environment};
use repository::TodoRepository;

mod app;
mod config;
mod repository;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("Failed to load config")?;

    let todo_repo = TodoRepository::new();
    if let Environment::Dev = config.env {
        todo_repo
            .add_examples()
            .context("Failed to add example todos")?;
    }

    let app = App::new(todo_repo);
    app.serve_api(&config).await?;

    Ok(())
}
