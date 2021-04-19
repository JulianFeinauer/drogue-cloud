use serde::{Deserialize, Serialize};

/// Authorize a request for a user.
///
/// NOTE: The user_id and roles information must come from a trusted source, like
/// a validated token. The user service will not re-validate this information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub application: String,

    pub user_id: String,
    pub roles: Vec<String>,
}

/// The result of an authorization request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthorizationResponse {
    /// The outcome, if the request.
    pub outcome: Outcome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Allow,
    Deny,
}

impl Outcome {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }

    pub fn ensure<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnOnce() -> E,
    {
        match self.is_allowed() {
            true => Ok(()),
            false => Err(f()),
        }
    }
}
