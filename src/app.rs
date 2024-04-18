use anyhow::{Context, Result};
use axum::{
    extract::State,
    routing::{get, post},
    serve, Form, Router,
};
use maud::{html, Markup};
use tokio::net::TcpListener;

use crate::{repository::TodoRepository, todo::Todo};

#[derive(Clone)]
pub(crate) struct App {
    todo_repo: TodoRepository,
}

impl App {
    pub fn new(todo_repo: TodoRepository) -> App {
        App { todo_repo }
    }

    pub async fn serve_api(self, port: u16) -> Result<()> {
        let router = Router::new()
            .route("/", get(index_page))
            .route("/todos", post(post_todos))
            .with_state(self);
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;
        serve(listener, router).await?;
        Ok(())
    }
}

async fn index_page(State(app): State<App>) -> ApiResult {
    let todos = app
        .todo_repo
        .get_todos()
        .context("Failed to get todos")
        .to_server_error()?;

    html! {
        head {
            title { "Rust HTMX Demo App" }
            script src="https://unpkg.com/htmx.org@1.9.12" {}
        }
        body {
            h1 { "Todos" }
            div { "Test" }
            ul #todos {
                @for todo in &todos {
                    (todo_item(todo))
                }
            }
            form {
                input type="text" name="content";
                input type="text" name="author";
                button hx-post="/todos" hx-target="#todos" hx-swap="afterbegin" {
                    "Create todo"
                }
            }
        }
    }
    .to_response()
}

async fn post_todos(State(app): State<App>, Form(todo): Form<Todo>) -> ApiResult {
    app.todo_repo
        .add_todo(&todo)
        .context("Failed to add todo")
        .to_server_error()?;

    todo_item(&todo).to_response()
}

fn todo_item(todo: &Todo) -> Markup {
    html! {
        li { (todo.author) ": " (todo.content) }
    }
}

use utils::*;
mod utils {
    use anyhow::Result;
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use maud::Markup;

    pub type ApiResult = Result<Response, Response>;

    pub trait ToApiError<T> {
        fn to_server_error(self) -> Result<T, Response>;
        fn to_client_error(self) -> Result<T, Response>;
    }

    impl<T> ToApiError<T> for Result<T> {
        fn to_server_error(self) -> Result<T, Response> {
            self.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
        }

        fn to_client_error(self) -> Result<T, Response> {
            self.map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())
        }
    }

    pub trait ToResponse {
        fn to_response(self) -> ApiResult;
    }

    impl ToResponse for Markup {
        fn to_response(self) -> ApiResult {
            Ok(self.into_response())
        }
    }
}
