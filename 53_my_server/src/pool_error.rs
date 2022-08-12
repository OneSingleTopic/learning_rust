use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum PoolError {
    PoolCreationError(String),
}

impl Display for PoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolError::PoolCreationError(message) => write!(f, "POOL CREATION ERROR [{}]", message),
        }
    }
}
impl Error for PoolError {}
