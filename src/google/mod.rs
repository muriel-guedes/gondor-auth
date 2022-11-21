use crypto::{sha2::Sha256, digest::Digest};
use rand::{seq::SliceRandom, thread_rng};

pub mod scopes;
mod builder;
pub use builder::*;

pub struct GoogleOAuth2 {
    pub id: String,
    pub secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<crate::Scope>,
    pub access_type: AccessType,
    pub include_granted_scopes: bool,
    pub state_size: usize
}
impl GoogleOAuth2 {
    pub fn get_scopes(&self) -> String {
        let mut res = Vec::with_capacity(self.scopes.len());
        for scope in &self.scopes {
            res.push(scope.0);
        }
        res.join("%20")
    }
    pub fn get_login_url(&self) -> String {
        let scopes = self.get_scopes();
        let access_type = self.access_type.to_str();
        let include_granted_scopes = if self.include_granted_scopes { "true" } else { "false" };

        let mut state = String::with_capacity(self.state_size);
        for v in crate::utils::CHARS.choose_multiple(&mut thread_rng(), self.state_size) {
            state.push(*v as char);
        }

        let mut hasher = Sha256::new();
        hasher.input_str(&state);
        let code_challenge = hasher.result_str();

        format!("\
https://accounts.google.com/o/oauth2/v2/auth?\
client_id={}&\
scope={scopes}&\
access_type={access_type}&\
include_granted_scopes={include_granted_scopes}&\
response_type=code&\
state={state}&\
code_challenge={code_challenge}&\
code_challenge_method=S256&\
redirect_uri={}\
", self.id, self.redirect_uri)
    }
}

pub enum AccessType {
    Online, Offline
}
impl AccessType {
    fn to_str(&self) -> &'static str {
        match self {
            AccessType::Offline => "offline",
            AccessType::Online => "online"
        }
    }
}