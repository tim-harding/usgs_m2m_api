use error_code::ErrorCode;
mod error_code;

const API_URL: &'static str = "https://m2m.cr.usgs.gov/api/api/json/";

#[derive(Debug, Clone, Copy, Hash)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct UserContext {
    contact_id: String,
    ip_address: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LoginInput {
    username: String,
    password: String,
    user_context: Option<UserContext>,
}


pub struct LoginOutput {
    error_code: ErrorCode,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
