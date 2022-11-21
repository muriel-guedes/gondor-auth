use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::Scope;
use super::AccessType;

pub struct GoogleOAuth2Builder {
    pub id: String,
    pub secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<Scope>,
    pub access_type: AccessType,
    pub include_granted_scopes: bool,
    pub state_size: usize
}
impl GoogleOAuth2Builder {
    pub fn new(
        id: impl Into<String>,
        secret: impl Into<String>,
        redirect_uri: impl AsRef<str>,
    ) -> Self {
        Self {
            id: id.into(),
            secret: secret.into(),
            redirect_uri: utf8_percent_encode(redirect_uri.as_ref(), NON_ALPHANUMERIC).to_string(),
            scopes: vec![],
            access_type: AccessType::Offline,
            include_granted_scopes: true,
            state_size: 32
        }
    }
    pub fn add_scope(mut self, v: Scope) -> Self {
        self.scopes.push(v);
        self
    }
    pub fn set_access_type(mut self, v: AccessType) -> Self {
        self.access_type = v;
        self
    }
    pub fn set_include_granted_scopes(mut self, v: bool) -> Self {
        self.include_granted_scopes = v;
        self
    }
    pub fn set_state_size(mut self, v: usize) -> Self {
        self.state_size = v;
        self
    }
    pub fn build(self) -> super::GoogleOAuth2 {
        super::GoogleOAuth2 {
            id: self.id,
            secret: self.secret,
            redirect_uri: self.redirect_uri,
            scopes: self.scopes,
            access_type: self.access_type,
            include_granted_scopes: self.include_granted_scopes,
            state_size: self.state_size
        }
    }
}