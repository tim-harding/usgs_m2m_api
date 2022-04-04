#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // General
    /// This endpoint is not available to the requested version
    EndpointUnavailable,
    /// An unknown error occurred
    Unknown,
    /// JSON payload could not be parsed as valid JSON
    InputFormat,
    /// Invalid parameters used
    InputParameterInvalid,
    /// Invalid parameters used
    InputInvalid,
    /// Couldn't find the input
    NotFound,
    /// API not configured to handle the request
    Server,
    /// Unknown version used
    VersionUnknown,

    // Authentication
    /// User credential verification failed
    AuthorizationInvalid,
    /// User account does not have access to the requested endpoint
    Unauthorized,
    /// Invalid API Key
    AuthorizationKeyInvalid,

    // Rate limit
    /// User attempted to run multiple requests at a time
    RateLimit,
    /// User has reached download-related rate limits
    RateLimitUserDownloadLimit,

    // Download
    /// Download does not belong to the user
    Download,

    // Export
    /// Unable to create metadata export
    Export,

    // Inventory
    /// This dataset does not support - full details in error message
    Dataset,
    /// Dataset is not available for the user - full details in error message
    DatasetUnauthorized,
    /// Dataset is not available for the user - full details in error message
    DatasetAuthorization,
    /// Invalid dataset used
    DatasetInvalid,
    /// Unable to clear dataset customization
    DatasetCustomClear,
    /// Unable to get dataset customization
    DatasetCustomizationGet,
    /// Unable to get dataset customizations
    DatasetCustomizationsGet,
    /// Unable to create or update dataset customization
    DatasetCustomizationSet,
    /// Unable to create or update dataset customization
    DatasetCustomizationsSet,
    /// Unable to create search records or unable to auto-execute the search request
    SearchCreate,
    /// Unable to execute search request
    Search,
    /// Unable to execute search request
    SearchExecute,
    /// Search failed
    SearchFailed,
    /// Unable to translate results into response format
    SearchResult,
    /// Search has not been completed
    SearchUnavailable,
    /// Unable to update the search - full details in the error message
    SearchUpdate,

    // Orders
    /// An order related error occurred - full details in error message
    Order,
    /// Order does not belong to the user
    OrderAuthorization,
    /// Invalid order given
    OrderInvalid,
    /// Unable to restore order units - full details in error message
    RestoreOrder,

    // Subscription
    /// Subscription creation failed
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
