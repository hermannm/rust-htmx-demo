use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
    pub content: String,
    pub author: String,
}

pub(crate) struct TodoErrors {
    pub content: Option<&'static str>,
    pub author: Option<&'static str>,
}

impl Todo {
    pub fn empty() -> Todo {
        Todo {
            content: "".to_string(),
            author: "".to_string(),
        }
    }

    pub fn validate(&self) -> Option<TodoErrors> {
        let mut errors = TodoErrors {
            content: None,
            author: None,
        };
        let mut is_err = false;

        if self.content.is_empty() {
            errors.content = Some("Todo cannot be blank");
            is_err = true;
        }
        if self.author.is_empty() {
            errors.author = Some("Name cannot be blank");
            is_err = true;
        }

        if is_err {
            Some(errors)
        } else {
            None
        }
    }
}
