use aws_sdk_s3::error::SdkError;
use axum::{
    body::Body,
    extract::multipart::MultipartError,
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

pub struct Error {
    status_code: StatusCode,
    message: String,
}

impl Error {
    pub fn new<S: Into<String>>(status: StatusCode, message: S) -> Self {
        Self {
            status_code: status,
            message: message.into(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self.status_code)
            .body(Body::from(self.message))
            .expect("failed to build error response")
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.kind() {
            jsonwebtoken::errors::ErrorKind::Json(_)
            | jsonwebtoken::errors::ErrorKind::Utf8(_)
            | jsonwebtoken::errors::ErrorKind::Base64(_)
            | jsonwebtoken::errors::ErrorKind::Crypto(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, "failed to parse jwt")
            }
            _ => Self::new(StatusCode::BAD_REQUEST, "invalid jwt"),
        }
    }
}

impl From<MultipartError> for Error {
    fn from(value: MultipartError) -> Self {
        Self::new(value.status(), value.to_string())
    }
}

impl<E, R> From<SdkError<E, R>> for Error {
    fn from(value: SdkError<E, R>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        if let Some(e) = value.as_database_error() {
            if let Some(code) = e.code() {
                return match code.to_string().as_str() {
                    "02000" => Self::new(StatusCode::NOT_FOUND, "no rows"),
                    "23502" => Self::new(StatusCode::BAD_REQUEST, "not null violation"),
                    "23503" => Self::new(StatusCode::NOT_FOUND, "foreign key violation"),
                    "23505" => Self::new(StatusCode::CONFLICT, "unique violation"),
                    "23514" => Self::new(StatusCode::BAD_REQUEST, "check violation"),
                    code => Self::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("unknown error code: {code}"),
                    ),
                };
            }
        };

        Self::new(StatusCode::INTERNAL_SERVER_ERROR, value.to_string())
    }
}
