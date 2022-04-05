mod data_types;
mod error_code;
mod errors;
mod login;

use login::LoginRawResponse;

pub use data_types::*;
pub use error_code::ErrorCode;
pub use errors::{ApiException, UsgsError};
pub use login::{LoginParameters, LoginResult};

const API_URL: &'static str = "https://m2m.cr.usgs.gov/api/api/json/";

pub struct Usgs {
    url: String,
}

impl Usgs {
    pub fn new(version: ApiVersion) -> Self {
        let version_string: &'static str = version.into();
        Self {
            url: format!("{}{}", API_URL, version_string),
        }
    }

    pub async fn query(&self) -> Result<LoginResult, UsgsError> {
        let username = std::env::var("USGS_USERNAME").unwrap();
        let password = std::env::var("USGS_PASSWORD").unwrap();
        let parameters = LoginParameters {
            username: &username,
            password: &password,
            ..Default::default()
        };
        let client = reqwest::Client::new();
        let response = client
            .post(&self.url)
            .json(&parameters)
            .send()
            .await?
            .json::<LoginRawResponse>()
            .await?;
        match response.error_code {
            Some(code) => match response.error_message {
                Some(message) => Err(UsgsError::ApiException(ApiException { code, message })),
                None => Err(UsgsError::ApiException(ApiException {
                    code,
                    message: "No message provided by API".to_string(),
                })),
            },
            None => Ok(LoginResult {
                token: response.data,
                request_id: response.request_id,
                session_id: response.session_id,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiVersion {
    Development,
    Experimental,
    Stable,
}

impl From<ApiVersion> for &'static str {
    fn from(version: ApiVersion) -> Self {
        match version {
            ApiVersion::Development => "development",
            ApiVersion::Experimental => "experimental",
            ApiVersion::Stable => "stable",
        }
    }
}
