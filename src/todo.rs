use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Todo<V: ValidationState> {
    pub content: String,
    pub author: String,
    #[serde(skip_serializing, default)]
    marker: PhantomData<V>,
}

#[derive(Clone, Debug)]
pub(crate) enum Validated {}
#[derive(Clone, Debug)]
pub(crate) enum NotValidated {}
pub(crate) trait ValidationState {}
impl ValidationState for Validated {}
impl ValidationState for NotValidated {}

#[derive(Debug)]
pub(crate) struct TodoErrors {
    pub content: Option<&'static str>,
    pub author: Option<&'static str>,
}

impl Todo<NotValidated> {
    pub fn empty() -> Self {
        Todo {
            content: "".to_string(),
            author: "".to_string(),
            marker: PhantomData,
        }
    }

    pub fn new(content: String, author: String) -> Self {
        Todo {
            content,
            author,
            marker: PhantomData,
        }
    }

    pub fn validate(self) -> Result<Todo<Validated>, (Todo<NotValidated>, TodoErrors)> {
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
            Err((self, errors))
        } else {
            Ok(Todo {
                content: self.content,
                author: self.author,
                marker: PhantomData,
            })
        }
    }
}
