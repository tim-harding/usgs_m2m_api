use crate::{data_types::UserContext, error_code::ErrorCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Serialize)]
pub struct LoginParameters<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub user_context: Option<UserContext<'a>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default, Deserialize)]
pub(crate) struct LoginRawResponse {
    pub(crate) error_code: Option<ErrorCode>,
    pub(crate) error_message: Option<String>,
    pub(crate) data: String,
    pub(crate) request_id: u64,
    pub(crate) session_id: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct LoginResult {
    pub token: String,
    pub request_id: u64,
    pub session_id: u64,
}
