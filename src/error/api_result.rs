use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::error;

pub struct ApiOk<T>(pub T);
pub struct ApiErr<T: std::error::Error + ErrorCode + std::fmt::Debug>(pub T);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommomApiResult<T>
where
    T: Serialize,
{
    pub result: bool,
    pub code: String,
    pub data: T,
    pub message: Option<String>,
    pub errors: Option<HashMap<String, Value>>,
}

impl<T> IntoResponse for CommomApiResult<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!(self)).into_response()
    }
}

impl<T> From<ApiOk<T>> for CommomApiResult<T>
where
    T: Serialize,
{
    fn from(value: ApiOk<T>) -> Self {
        CommomApiResult {
            result: true,
            code: "00".to_string(),
            data: value.0,
            message: Some("Success".to_string()),
            errors: None,
        }
    }
}

impl<T> IntoResponse for ApiOk<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        CommomApiResult::from(self).into_response()
    }
}

pub struct WrappedResponse(pub axum::response::Response);
impl IntoResponse for WrappedResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

impl IntoResponse for ApiOk<WrappedResponse> {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

pub trait ErrorCode {
    fn error_code(&self) -> String;
    fn data(&self) -> Value {
        Value::Null
    }

    fn message(&self) -> String {
        "".to_string()
    }

    fn errors(&self) -> HashMap<String, Value> {
        HashMap::new()
    }
}

impl<T> std::fmt::Debug for ApiErr<T>
where
    T: std::error::Error + ErrorCode + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> IntoResponse for ApiErr<T>
where
    T: std::error::Error + ErrorCode + std::fmt::Debug,
{
    fn into_response(self) -> axum::response::Response {
        error!("ApiErr:\n{:?}", self);
        CommomApiResult {
            result: false,
            data: Some(self.0.data()),
            code: self.0.error_code(),
            message: Some(self.0.message()),
            errors: Some(self.0.errors()),
        }
        .into_response()
    }
}

impl<T> From<T> for ApiErr<T>
where
    T: std::error::Error + ErrorCode,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[macro_export]
macro_rules! api_ok {
    ($data:expr) => {
        Ok($crate::error::api_result::ApiOk($data))
    };
}