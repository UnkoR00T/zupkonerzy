use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}
