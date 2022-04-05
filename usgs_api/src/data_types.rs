use serde::Serialize;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UserContext<'a> {
    contact_id: &'a str,
    ip_address: &'a str,
}