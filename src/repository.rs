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

    pub fn add_todo(&self, todo: &Todo) -> Result<()> {
        let mut todos = self
            .todos
            .write()
            .map_err(|err| anyhow!("Failed to acquire lock on todos: {err}"))?;
        todos.push(todo.clone());
        Ok(())
    }
}
