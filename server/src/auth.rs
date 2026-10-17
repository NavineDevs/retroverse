//! Accounts and sign-in. Not implemented in this base project.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u32,
}

/// Always `Err`: there is no account store and nothing to check a password against.
pub fn sign_in(_request: &SignInRequest) -> Result<Token, String> {
    Err("Not implemented in the RetroVerse base project.".into())
}
