use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ErrorCode {
    // General
    /// This endpoint is not available to the requested version
    #[serde(rename(
        serialize = "ENDPOINT_UNAVAILABLE",
        deserialize = "ENDPOINT_UNAVAILABLE"
    ))]
    EndpointUnavailable,
    /// An unknown error occurred
    #[serde(rename(serialize = "UNKNOWN", deserialize = "UNKNOWN"))]
    Unknown,
    /// JSON payload could not be parsed as valid JSON
    #[serde(rename(serialize = "INPUT_FORMAT", deserialize = "INPUT_FORMAT"))]
    InputFormat,
    /// Invalid parameters used
    #[serde(rename(
        serialize = "INPUT_PARAMETER_INVALID",
        deserialize = "INPUT_PARAMETER_INVALID"
    ))]
    InputParameterInvalid,
    /// Invalid parameters used
    #[serde(rename(serialize = "INPUT_INVALID", deserialize = "INPUT_INVALID"))]
    InputInvalid,
    /// Couldn't find the input
    #[serde(rename(serialize = "NOT_FOUND", deserialize = "NOT_FOUND"))]
    NotFound,
    /// API not configured to handle the request
    #[serde(rename(serialize = "SERVER_ERROR", deserialize = "SERVER_ERROR"))]
    Server,
    /// Unknown version used
    #[serde(rename(serialize = "VERSION_UNKNOWN", deserialize = "VERSION_UNKNOWN"))]
    VersionUnknown,

    // Authentication
    /// User credential verification failed
    #[serde(rename(serialize = "AUTH_INVALID", deserialize = "AUTH_INVALID"))]
    AuthorizationInvalid,
    /// User account does not have access to the requested endpoint
    #[serde(rename(serialize = "AUTH_UNAUTHORIZED", deserialize = "AUTH_UNAUTHORIZED"))]
    Unauthorized,
    /// Invalid API Key
    #[serde(rename(serialize = "AUTH_KEY_INVALID", deserialize = "AUTH_KEY_INVALID"))]
    AuthorizationKeyInvalid,

    // Rate limit
    /// User attempted to run multiple requests at a time
    #[serde(rename(serialize = "RATE_LIMIT", deserialize = "RATE_LIMIT"))]
    RateLimit,
    /// User has reached download-related rate limits
    #[serde(rename(serialize = "RATE_LIMIT_USER_DL", deserialize = "RATE_LIMIT_USER_DL"))]
    RateLimitUserDownloadLimit,

    // Download
    /// Download does not belong to the user
    #[serde(rename(serialize = "DOWNLOAD_ERROR", deserialize = "DOWNLOAD_ERROR"))]
    Download,

    // Export
    /// Unable to create metadata export
    #[serde(rename(serialize = "EXPORT_ERROR", deserialize = "EXPORT_ERROR"))]
    Export,

    // Inventory
    /// This dataset does not support - full details in error message
    #[serde(rename(serialize = "DATASET_ERROR", deserialize = "DATASET_ERROR"))]
    Dataset,
    /// Dataset is not available for the user - full details in error message
    #[serde(rename(
        serialize = "DATASET_UNAUTHORIZED",
        deserialize = "DATASET_UNAUTHORIZED"
    ))]
    DatasetUnauthorized,
    /// Dataset is not available for the user - full details in error message
    #[serde(rename(serialize = "DATASET_AUTH", deserialize = "DATASET_AUTH"))]
    DatasetAuthorization,
    /// Invalid dataset used
    #[serde(rename(serialize = "DATASET_INVALID", deserialize = "DATASET_INVALID"))]
    DatasetInvalid,
    /// Unable to clear dataset customization
    #[serde(rename(
        serialize = "DATASET_CUSTOM_CLEAR_ERROR",
        deserialize = "DATASET_CUSTOM_CLEAR_ERROR"
    ))]
    DatasetCustomClear,
    /// Unable to get dataset customization
    #[serde(rename(
        serialize = "DATASET_CUSTOM_GET_ERROR",
        deserialize = "DATASET_CUSTOM_GET_ERROR"
    ))]
    DatasetCustomizationGet,
    /// Unable to get dataset customizations
    #[serde(rename(
        serialize = "DATASET_CUSTOMS_GET_ERROR",
        deserialize = "DATASET_CUSTOMS_GET_ERROR"
    ))]
    DatasetCustomizationsGet,
    /// Unable to create or update dataset customization
    #[serde(rename(
        serialize = "DATASET_CUSTOM_SET_ERROR",
        deserialize = "DATASET_CUSTOM_SET_ERROR"
    ))]
    DatasetCustomizationSet,
    /// Unable to create or update dataset customization
    #[serde(rename(
        serialize = "DATASET_CUSTOMS_SET_ERROR",
        deserialize = "DATASET_CUSTOMS_SET_ERROR"
    ))]
    DatasetCustomizationsSet,
    /// Unable to create search records or unable to auto-execute the search request
    #[serde(rename(serialize = "SEARCH_CREATE_ERROR", deserialize = "SEARCH_CREATE_ERROR"))]
    SearchCreate,
    /// Unable to execute search request
    #[serde(rename(serialize = "SEARCH_ERROR", deserialize = "SEARCH_ERROR"))]
    Search,
    /// Unable to execute search request
    #[serde(rename(
        serialize = "SEARCH_EXECUTE_ERROR",
        deserialize = "SEARCH_EXECUTE_ERROR"
    ))]
    SearchExecute,
    /// Search failed
    #[serde(rename(serialize = "SEARCH_FAILED", deserialize = "SEARCH_FAILED"))]
    SearchFailed,
    /// Unable to translate results into response format
    #[serde(rename(serialize = "SEARCH_RESULT_ERROR", deserialize = "SEARCH_RESULT_ERROR"))]
    SearchResult,
    /// Search has not been completed
    #[serde(rename(serialize = "SEARCH_UNAVAILABLE", deserialize = "SEARCH_UNAVAILABLE"))]
    SearchUnavailable,
    /// Unable to update the search - full details in the error message
    #[serde(rename(serialize = "SEARCH_UPDATE_ERROR", deserialize = "SEARCH_UPDATE_ERROR"))]
    SearchUpdate,

    // Orders
    /// An order related error occurred - full details in error message
    #[serde(rename(serialize = "ORDER_ERROR", deserialize = "ORDER_ERROR"))]
    Order,
    /// Order does not belong to the user
    #[serde(rename(serialize = "ORDER_AUTH", deserialize = "ORDER_AUTH"))]
    OrderAuthorization,
    /// Invalid order given
    #[serde(rename(serialize = "ORDER_INVALID", deserialize = "ORDER_INVALID"))]
    OrderInvalid,
    /// Unable to restore order units - full details in error message
    #[serde(rename(serialize = "RESTORE_ORDER_ERROR", deserialize = "RESTORE_ORDER_ERROR"))]
    RestoreOrder,

    // Subscription
    /// Subscription creation failed
    #[serde(rename(serialize = "SUBSCRIPTION_ERROR", deserialize = "SUBSCRIPTION_ERROR"))]
    Subscription,

    /// The code received is not documented
    Other,
}

impl From<&str> for ErrorCode {
    fn from(code: &str) -> Self {
        match code {
            "ENDPOINT_UNAVAILABLE" => Self::EndpointUnavailable,
            "UNKNOWN" => Self::Unknown,
            "INPUT_FORMAT" => Self::InputFormat,
            "INPUT_PARAMETER_INVALID" => Self::InputParameterInvalid,
            "INPUT_INVALID" => Self::InputInvalid,
            "NOT_FOUND" => Self::NotFound,
            "SERVER_ERROR" => Self::Server,
            "VERSION_UNKNOWN" => Self::VersionUnknown,
            "AUTH_INVALID" => Self::AuthorizationInvalid,
            "AUTH_UNAUTHORIZED" => Self::Unauthorized,
            "AUTH_KEY_INVALID" => Self::AuthorizationKeyInvalid,
            "RATE_LIMIT" => Self::RateLimit,
            "RATE_LIMIT_USER_DL" => Self::RateLimitUserDownloadLimit,
            "DOWNLOAD_ERROR" => Self::Download,
            "EXPORT_ERROR" => Self::Export,
            "DATASET_ERROR" => Self::Dataset,
            "DATASET_UNAUTHORIZED" => Self::DatasetUnauthorized,
            "DATASET_AUTH" => Self::DatasetAuthorization,
            "DATASET_INVALID" => Self::DatasetInvalid,
            "DATASET_CUSTOM_CLEAR_ERROR" => Self::DatasetCustomClear,
            "DATASET_CUSTOM_GET_ERROR" => Self::DatasetCustomizationGet,
            "DATASET_CUSTOMS_GET_ERROR" => Self::DatasetCustomizationsGet,
            "DATASET_CUSTOM_SET_ERROR" => Self::DatasetCustomizationSet,
            "DATASET_CUSTOMS_SET_ERROR" => Self::DatasetCustomizationsSet,
            "SEARCH_CREATE_ERROR" => Self::SearchCreate,
            "SEARCH_ERROR" => Self::Search,
            "SEARCH_EXECUTE_ERROR" => Self::SearchExecute,
            "SEARCH_FAILED" => Self::SearchFailed,
            "SEARCH_RESULT_ERROR" => Self::SearchResult,
            "SEARCH_UNAVAILABLE" => Self::SearchUnavailable,
            "SEARCH_UPDATE_ERROR" => Self::SearchUpdate,
            "ORDER_ERROR" => Self::Order,
            "ORDER_AUTH" => Self::OrderAuthorization,
            "ORDER_INVALID" => Self::OrderInvalid,
            "RESTORE_ORDER_ERROR" => Self::RestoreOrder,
            "SUBSCRIPTION_ERROR" => Self::Subscription,
            _ => Self::Other,
        }
    }
}

impl From<&ErrorCode> for &'static str {
    fn from(code: &ErrorCode) -> Self {
        match code {
            ErrorCode::EndpointUnavailable => "ENDPOINT_UNAVAILABLE",
            ErrorCode::Unknown => "UNKNOWN",
            ErrorCode::InputFormat => "INPUT_FORMAT",
            ErrorCode::InputParameterInvalid => "INPUT_PARAMETER_INVALID",
            ErrorCode::InputInvalid => "INPUT_INVALID",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::Server => "SERVER_ERROR",
            ErrorCode::VersionUnknown => "VERSION_UNKNOWN",
            ErrorCode::AuthorizationInvalid => "AUTH_INVALID",
            ErrorCode::Unauthorized => "AUTH_UNAUTHORIZED",
            ErrorCode::AuthorizationKeyInvalid => "AUTH_KEY_INVALID",
            ErrorCode::RateLimit => "RATE_LIMIT",
            ErrorCode::RateLimitUserDownloadLimit => "RATE_LIMIT_USER_DL",
            ErrorCode::Download => "DOWNLOAD_ERROR",
            ErrorCode::Export => "EXPORT_ERROR",
            ErrorCode::Dataset => "DATASET_ERROR",
            ErrorCode::DatasetUnauthorized => "DATASET_UNAUTHORIZED",
            ErrorCode::DatasetAuthorization => "DATASET_AUTH",
            ErrorCode::DatasetInvalid => "DATASET_INVALID",
            ErrorCode::DatasetCustomClear => "DATASET_CUSTOM_CLEAR_ERROR",
            ErrorCode::DatasetCustomizationGet => "DATASET_CUSTOM_GET_ERROR",
            ErrorCode::DatasetCustomizationsGet => "DATASET_CUSTOMS_GET_ERROR",
            ErrorCode::DatasetCustomizationSet => "DATASET_CUSTOM_SET_ERROR",
            ErrorCode::DatasetCustomizationsSet => "DATASET_CUSTOMS_SET_ERROR",
            ErrorCode::SearchCreate => "SEARCH_CREATE_ERROR",
            ErrorCode::Search => "SEARCH_ERROR",
            ErrorCode::SearchExecute => "SEARCH_EXECUTE_ERROR",
            ErrorCode::SearchFailed => "SEARCH_FAILED",
            ErrorCode::SearchResult => "SEARCH_RESULT_ERROR",
            ErrorCode::SearchUnavailable => "SEARCH_UNAVAILABLE",
            ErrorCode::SearchUpdate => "SEARCH_UPDATE_ERROR",
            ErrorCode::Order => "ORDER_ERROR",
            ErrorCode::OrderAuthorization => "ORDER_AUTH",
            ErrorCode::OrderInvalid => "ORDER_INVALID",
            ErrorCode::RestoreOrder => "RESTORE_ORDER_ERROR",
            ErrorCode::Subscription => "SUBSCRIPTION_ERROR",
            ErrorCode::Other => "Undocumented error code",
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static str = self.into();
        write!(f, "{}", s)
    }
}
