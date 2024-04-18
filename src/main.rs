use anyhow::Result;
use app::App;
use repository::TodoRepository;

mod app;
mod repository;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    let todo_repo = TodoRepository::new();
    let app = App::new(todo_repo);
    app.serve_api(8000).await?;
    Ok(())
}
