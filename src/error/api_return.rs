use std::{collections::HashMap, result};

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;
use snafu::Snafu;

use super::result_code::ResultCode;


pub struct AppSuccess<T>(pub T);
pub type Result<T, E = AppError> = result::Result<T, E>;
pub const ERROR: &str = "error";

#[derive(Serialize)]
pub struct ApiReturn<T>
where 
    T: Serialize
{
    pub result: bool,
    pub code: String,
    pub data: T,
    pub message: Option<String>,
    pub errors: Option<HashMap<String, String>>,
}

impl<T> IntoResponse for ApiReturn<T> 
where 
    T: serde::Serialize
{
    fn into_response(self) -> axum::response::Response {
        Json(serde_json::json!(self)).into_response()
    }
}

impl <T> From<AppSuccess<T>> for ApiReturn<T> 
where 
    T: Serialize
{
    fn from(value: AppSuccess<T>) -> Self {
        ApiReturn {
            result: true,
            code: "00".to_string(),
            data: value.0,
            message: Some("Success".to_string()),
            errors: None,
        }
    }
}

impl<T> IntoResponse for AppSuccess<T> 
where 
    T: serde::Serialize
{
    fn into_response(self) -> axum::response::Response {
        ApiReturn::from(self).into_response()
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum AppError {
    #[snafu(display("Data not found. {message}"))]
    DataNotFound { message: String },

    #[snafu(display("Other database operator error. {message}"))]
    OtherDbErr { message: String },

    #[snafu(display("Meta database access error. {source:?}"))]
    MetaDataBaseAccessErr { source: DbErr },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let result_code = Self::get_result_code(&self);
        let errors = HashMap::from([
                    (ERROR.to_owned(), self.to_string()),
                ]);
        let api_return = ApiReturn {
                    result: false,
                    data: Some(()),
                    code: result_code.code().to_string(),
                    message: Some(result_code.message().to_string()),
                    errors: Some(errors),
                };
        (StatusCode::OK, Json(json!(api_return))).into_response()
    }
}

impl AppError {
    fn get_result_code(&self) -> ResultCode {
        match self {
            Self::DataNotFound { .. } => ResultCode::DataNotFound,
            Self::OtherDbErr { .. } => ResultCode::OtherDbErr,
            _ => ResultCode::OtherErr,
        }
    }
}

#[macro_export]
macro_rules! success {
    ($data:expr) => {
        Ok($crate::error::api_return::AppSuccess($data))
    };
}


