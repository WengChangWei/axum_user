use std::collections::HashMap;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use sea_orm::DbErr;
use serde_json::json;
use snafu::Snafu;
use std::result;

use super::{api_reponse::ApiResponse, api_result::{ApiErr, ApiOk}, result_code::ResultCode};

pub const ERROR: &str = "error";

pub type Result<T, E = AppError> = result::Result<T, E>;
pub type ApiResult<A, B = AppError> = result::Result<ApiOk<A>, ApiErr<B>>;

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
        let api_response: ApiResponse<&str> = ApiResponse::error(
                    Some(result_code.message()), 
                    Some(&result_code), 
                    Some(errors)
                );
        (StatusCode::OK, Json(json!(api_response))).into_response()
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
