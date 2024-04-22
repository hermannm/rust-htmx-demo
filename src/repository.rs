use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};

use crate::todo::{Todo, Validated};

#[derive(Clone)]
pub(crate) struct TodoRepository {
    todos: Arc<RwLock<Vec<Todo<Validated>>>>,
}

impl TodoRepository {
    pub fn new() -> TodoRepository {
        TodoRepository {
            todos: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get_todos(&self) -> Result<Vec<Todo<Validated>>> {
        let todos = self
            .todos
            .read()
            .map_err(|err| anyhow!("Failed to acquire lock on todos: {err}"))?;
        Ok(todos
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<Todo<Validated>>>())
    }

    pub fn add_todo(&self, todo: Todo<Validated>) -> Result<()> {
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
        let examples = [
            Todo::new(
                "Initialize Cargo project".to_string(),
                "hermannm".to_string(),
            ),
            Todo::new("Create basic todo app".to_string(), "hermannm".to_string()),
            Todo::new("Set up Tailwind CSS".to_string(), "hermannm".to_string()),
            Todo::new("Add example todos".to_string(), "hermannm".to_string()),
        ];
        todos.extend_from_slice(&examples.map(|example| example.validate().unwrap()));
        Ok(())
    }
}
