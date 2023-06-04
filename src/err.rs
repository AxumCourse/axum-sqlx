use axum::response::IntoResponse;

#[derive(Debug)]
pub enum ErrorKind {
    Database,
    Config,
    Template,
    NotFound,
    Exists,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    pub fn new(
        kind: ErrorKind,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
    ) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }

    pub fn with_cause(cause: Box<dyn std::error::Error>, kind: ErrorKind) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }

    pub fn not_found(msg: &str) -> Self {
        Self::new(ErrorKind::NotFound, msg.to_string(), None)
    }

    pub fn exists(msg: &str) -> Self {
        Self::new(ErrorKind::Exists, msg.to_string(), None)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::with_cause(Box::new(e), ErrorKind::Database)
    }
}

impl From<config::ConfigError> for Error {
    fn from(e: config::ConfigError) -> Self {
        Self::with_cause(Box::new(e), ErrorKind::Config)
    }
}
impl From<askama::Error> for Error {
    fn from(e: askama::Error) -> Self {
        Self::with_cause(Box::new(e), ErrorKind::Template)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        self.message.into_response()
    }
}
