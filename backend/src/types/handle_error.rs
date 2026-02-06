use std::fmt::Debug;

pub enum HandleError {
    ServerError(u8, String),
    HandlingError(u8, String),
}

impl Debug for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandleError::ServerError(code, msg) => write!(f, "ServerError({}: {})", code, msg),
            HandleError::HandlingError(code, msg) => write!(f, "HandlingError({}: {})", code, msg),
        }
    }
}
