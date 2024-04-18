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
