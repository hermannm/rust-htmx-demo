use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};

use crate::todo::Todo;

#[derive(Clone)]
pub(crate) struct TodoRepository {
    todos: Arc<RwLock<Vec<Todo>>>,
}

impl TodoRepository {
    pub fn new() -> TodoRepository {
        TodoRepository {
            todos: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = self
            .todos
            .read()
            .map_err(|err| anyhow!("Failed to acquire lock on todos: {err}"))?;
        Ok(todos.iter().rev().cloned().collect::<Vec<Todo>>())
    }

    pub fn add_todo(&self, todo: Todo) -> Result<()> {
        let mut todos = self
            .todos
            .write()
            .map_err(|err| anyhow!("Failed to acquire lock on todos: {err}"))?;
        todos.push(todo);
        Ok(())
    }

    pub fn add_examples(&self) -> Result<()> {
        let mut todos = self
            .todos
            .write()
            .map_err(|err| anyhow!("Failed to acquire lock on todos: {err}"))?;
        todos.extend_from_slice(&[
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
        ]);
        Ok(())
    }
}
