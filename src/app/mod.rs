use anyhow::{Context, Result};
use axum::{
    extract::State,
    routing::{get, post},
    serve, Form, Router,
};
use maud::{html, Markup};
use tokio::net::TcpListener;
use tower_livereload::LiveReloadLayer;

use crate::{
    config::{Config, Environment},
    repository::TodoRepository,
    todo::Todo,
};

mod static_assets;
mod utils;
use utils::*;

use self::static_assets::static_handler;

#[derive(Clone)]
pub(crate) struct App {
    todo_repo: TodoRepository,
}

impl App {
    pub fn new(todo_repo: TodoRepository) -> App {
        App { todo_repo }
    }

    pub async fn serve_api(self, config: &Config) -> Result<()> {
        let mut router = Router::new()
            .route("/static/*file", get(static_handler))
            .route("/", get(index_page))
            .route("/todos", post(post_todos))
            .with_state(self);
        if let Environment::Dev = config.environment {
            router = router.layer(LiveReloadLayer::new());
        }

        let listener = TcpListener::bind(("0.0.0.0", config.port)).await?;
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
            script src="/static/htmx-1.9.12.js" {}
            link rel="stylesheet" href="/static/styles.css";
        }
        body class="max-w-3xl mx-auto my-5 bg-slate-300" {
            h1 class="text-2xl flex justify-center" {
                "Todos"
            }
            div class="flex justify-center" {
                form class="flex flex-col gap-3 max-w-96" {
                    div class="flex flex-col gap-1" {
                        label class="font-bold" for="content" {
                            "Todo:"
                        }
                        textarea name="content" cols="40" rows="5" class="border border-gray-700" {}
                    }
                    div class="flex gap-1" {
                        label class="font-bold" for="author" {
                            "Your name:"
                        }
                        input type="text" name="author" class="border border-gray-700 flex-grow";
                    }
                    div class="flex justify-center" {
                        button hx-post="/todos" hx-target="#todos" hx-swap="afterbegin" type="submit"
                            class="bg-blue-600 p-2 rounded text-white" {
                            "Create todo"
                        }
                    }
                }
            }
            ul #todos class="grid grid-cols-3 gap-4" {
                @for todo in &todos {
                    (todo_item(todo))
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
        li class="bg-slate-700 text-white p-2 flex flex-col gap-4" {
            div class="flex-grow" {
                (todo.content)
            }
            div class="flex justify-end text-gray-400" {
                (todo.author)
            }
        }
    }
}
