use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub jti: Uuid,
}
