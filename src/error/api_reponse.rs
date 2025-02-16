use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::error::result_code::ResultCode::{Success, OtherErr};

use super::result_code::ResultCode;

pub const OK: &str = "ok";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse<'a, T> {
    pub result: bool,
    pub message: &'a str,
    pub code: &'a str,
    pub data: T,
    pub errors: HashMap<String, String>
}

impl Default for ApiResponse<'_, String> {
    fn default() -> Self {
        let api_resp = Self {
            result: true,
            message: Success.message(),
            code: Success.code(),
            data: OK.to_string(),
            errors: Default::default(),
        };
        api_resp
    }
}


impl<'a, T> ApiResponse<'a, T>
where
    T: Default,
{
    pub fn success(data: Option<T>, message: Option<&'a str>) -> Self {
        let api_resp = Self {
            result: true,
            message: message.unwrap_or(Success.message()),
            code: Success.code(),
            data: data.unwrap_or_default(),
            errors: Default::default(),
        };
        api_resp
    }

    pub fn error(
        message: Option<&'a str>,
        result_code: Option<&'a ResultCode>,
        errors: Option<HashMap<String, String>>,
    ) -> Self {
        let api_resp = Self {
            result: false,
            message: message.unwrap_or(result_code.unwrap_or(&OtherErr).message()),
            code: result_code.unwrap_or(&OtherErr).code(),
            data: Default::default(),
            errors: errors.unwrap_or_default(),
        };
        api_resp
    }
}