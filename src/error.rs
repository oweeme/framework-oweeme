use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum FrameworkError {
    Template(tera::Error),
    Api(reqwest::Error),
    NotFound(String),
    Internal(anyhow::Error),
}

impl std::fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameworkError::Template(e) => write!(f, "Template error: {e}"),
            FrameworkError::Api(e) => write!(f, "API error: {e}"),
            FrameworkError::NotFound(msg) => write!(f, "Not found: {msg}"),
            FrameworkError::Internal(e) => write!(f, "Internal error: {e}"),
        }
    }
}

impl IntoResponse for FrameworkError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            FrameworkError::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, msg).into_response()
    }
}

impl From<tera::Error> for FrameworkError {
    fn from(e: tera::Error) -> Self { FrameworkError::Template(e) }
}

impl From<reqwest::Error> for FrameworkError {
    fn from(e: reqwest::Error) -> Self { FrameworkError::Api(e) }
}

impl From<anyhow::Error> for FrameworkError {
    fn from(e: anyhow::Error) -> Self { FrameworkError::Internal(e) }
}
