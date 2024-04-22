use anyhow::{anyhow, Context, Result};
use axum::{
    extract::State,
    http::Request,
    response::{sse::Event, Sse},
    routing::{get, post},
    serve, Form, Router,
};
use futures::Stream;
use maud::{html, Markup};
use tokio::{
    net::TcpListener,
    sync::broadcast::{self, Sender},
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tower_http::trace::TraceLayer;
use tower_livereload::LiveReloadLayer;
use tracing::{info, Level};

use crate::{
    config::{Config, Environment},
    repository::TodoRepository,
    todo::{NotValidated, Todo, TodoErrors, Validated, ValidationState},
};

mod static_assets;
mod utils;
use utils::*;

use self::static_assets::static_handler;

#[derive(Clone)]
pub(crate) struct App {
    todo_repo: TodoRepository,
    todo_channel: Sender<Todo<Validated>>,
}

impl App {
    pub fn new(todo_repo: TodoRepository) -> App {
        let (todo_channel, _) = broadcast::channel::<Todo<Validated>>(256);
        App {
            todo_repo,
            todo_channel,
        }
    }

    pub async fn serve_api(self, config: &Config) -> Result<()> {
        let mut router = Router::new()
            .route("/static/*file", get(static_handler))
            .route("/", get(index_page))
            .route("/todos", post(post_todo))
            .route("/todo-stream", get(todo_stream))
            .with_state(self)
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    tracing::span!(
                        Level::DEBUG,
                        "request",
                        path = %format!("{} {}", request.method(), request.uri()),
                    )
                }),
            );
        if let Environment::Dev = config.environment {
            router = router.layer(LiveReloadLayer::new());
        }

        let listener = TcpListener::bind(("0.0.0.0", config.port)).await?;
        info!(port = config.port, env = ?config.environment, "Serving app");
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
            script src="/static/htmx-sse-1.9.12.js" {}
            script defer src="/static/script.js" {}
            link rel="stylesheet" href="/static/styles.css";
        }
        body class="max-w-3xl mx-auto my-5 bg-slate-300" {
            h1 class="text-2xl flex justify-center" {
                "Todos"
            }
            div class="flex justify-center" {
                (todo_form(&Todo::empty(), None))
            }
            ul id="todos" hx-ext="sse" sse-connect="/todo-stream" sse-swap="message"
                hx-swap="afterbegin" class="grid grid-cols-3 gap-4" {
                @for todo in &todos {
                    (todo_item(todo))
                }
            }
            (error_popups())
        }
    }
    .to_response()
}

async fn post_todo(State(app): State<App>, Form(todo): Form<Todo<NotValidated>>) -> ApiResult {
    let validated_todo = match todo.validate() {
        Ok(todo) => todo,
        Err((todo, errors)) => {
            return todo_form(&todo, Some(errors)).to_response();
        }
    };

    app.todo_repo
        .add_todo(validated_todo.clone())
        .context("Failed to add todo")
        .to_server_error()?;
    app.todo_channel
        .send(validated_todo.clone())
        .map_err(|err| anyhow!("Failed to send new todo on channel: {err}"))
        .to_server_error()?;

    todo_form(&Todo::new("".to_string(), validated_todo.author), None).to_response()
}

async fn todo_stream(State(app): State<App>) -> Sse<impl Stream<Item = Result<Event>>> {
    let receiver = app.todo_channel.subscribe();

    Sse::new(
        BroadcastStream::new(receiver).map(|receive_result| -> Result<Event> {
            let todo = receive_result.context("Failed to receive todo from channel")?;
            Ok(Event::default().data(todo_item(&todo).into_string()))
        }),
    )
}

fn todo_form(form_data: &Todo<impl ValidationState>, errors: Option<TodoErrors>) -> Markup {
    html! {
        form id="todo-form" hx-swap-oob="true" class="flex flex-col gap-3 max-w-96" {
            div class="flex flex-col gap-1" {
                label for="content" class="font-bold" {
                    "Todo:"
                }
                textarea id="todo-content-input" name="content" cols="40" rows="5"
                    value=(form_data.content) class="border border-gray-700 p-2 rounded" {}
                @if let Some(content_err) = errors.as_ref().and_then(|errors| errors.content) {
                    div class="text-red-600 font-bold flex justify-center" {
                        (content_err)
                    }
                }
            }
            div class="flex flex-col gap-1" {
                div class="flex gap-1 items-center" {
                    label for="author" class="font-bold" {
                        "Your name:"
                    }
                    input type="text" name="author" value=(form_data.author)
                        class="border border-gray-700 flex-grow p-2 rounded";
                }
                @if let Some(author_err) = errors.as_ref().and_then(|errors| errors.author) {
                    div class="text-red-600 font-bold flex justify-center" {
                        (author_err)
                    }
                }
            }
            div class="flex justify-center" {
                button hx-post="/todos" hx-disabled-elt="this"
                    class="bg-blue-600 disabled:bg-gray-500 p-2 rounded text-white" {
                    "Create todo"
                }
            }
        }
    }
}

fn todo_item(todo: &Todo<impl ValidationState>) -> Markup {
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

fn error_popups() -> Markup {
    html! {
        div id="error-popups" class="absolute bottom-4 right-4 flex flex-col gap-4" {}
        template id="error-popup" {
            div class="bg-red-700 text-white flex flex-col gap-3 p-3 min-w-64" {
                div class="flex gap-1 font-bold" {
                    div class="flex-grow" {
                        "Error"
                    }
                    button id="error-popup-closer" {
                        "X"
                    }
                }
                pre id="error-popup-content" {}
            }
        }
    }
}
