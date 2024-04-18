use anyhow::{Context, Result};
use app::App;
use repository::TodoRepository;

mod app;
mod repository;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    let todo_repo = TodoRepository::new();
    todo_repo
        .add_examples()
        .context("Failed to add example todos")?;

    let app = App::new(todo_repo);
    app.serve_api(8000).await?;

    Ok(())
}
