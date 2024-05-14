use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, todo::Todo};

#[derive(Clone)]
pub(crate) struct TodoDatabase {
    conn: Pool<Postgres>,
}

impl TodoDatabase {
    pub async fn new(config: &Config) -> Result<TodoDatabase> {
        let pool = PgPoolOptions::new()
            .connect(&config.database_url)
            .await
            .context("Failed to connect to Postgres database")?;

        Ok(TodoDatabase { conn: pool })
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = sqlx::query_as!(Todo, "SELECT author, content FROM todos ORDER BY id DESC")
            .fetch_all(&self.conn)
            .await?;
        Ok(todos)
    }

    pub async fn add_todo(&self, todo: Todo) -> Result<()> {
        sqlx::query!(
            "INSERT INTO todos (author, content) VALUES ($1, $2)",
            todo.author,
            todo.content
        )
        .execute(&self.conn)
        .await?;
        Ok(())
    }

    pub async fn add_examples(&self) -> Result<()> {
        let example_todos = [
            Todo {
                content: "Initialize Cargo project".to_string(),
                author: "hermannm".to_string(),
            },
            Todo {
                content: "Create basic todo app".to_string(),
                author: "hermannm".to_string(),
            },
            Todo {
                content: "Set up Tailwind CSS".to_string(),
                author: "hermannm".to_string(),
            },
            Todo {
                content: "Add example todos".to_string(),
                author: "hermannm".to_string(),
            },
        ];

        for todo in example_todos {
            self.add_todo(todo).await?;
        }
        Ok(())
    }
}
