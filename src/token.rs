use jsonwebtoken::dangerous_insecure_decode;
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct Token {
    id: String,
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    iat: u64, // issued_at
    exp: u64, // expiration time
}

impl Token {
    pub fn new(id: String, access_token: String, refresh_token: String) -> Token {
        Token {
            id,
            access_token,
            refresh_token,
        }
    }

    pub fn id(self) -> String {
        self.id
    }

    pub fn exp(self) -> u64 {
        match dangerous_insecure_decode::<Claims>(&self.access_token) {
            Ok(decoded) => decoded.claims.exp,
            Err(e) => {
                warn!("error decoding JWT token: {}", e);

                0
            }
        }
    }

    pub fn iat(self) -> u64 {
        match dangerous_insecure_decode::<Claims>(&self.access_token) {
            Ok(decoded) => decoded.claims.iat,
            Err(e) => {
                warn!("error decoding JWT token: {}", e);

                0
            }
        }
    }

    pub fn access_token(self) -> String {
        self.access_token
    }
}
