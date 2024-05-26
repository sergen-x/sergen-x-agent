use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub(crate) enum SergenError {
    InstallationError(String),
}

impl std::error::Error for SergenError {}

impl std::fmt::Display for SergenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SergenError::InstallationError(msg) => write!(f, "Error while installing: {}", msg),
        }
    }
}

// Convert reqwest::Error to SergenError
impl From<ReqwestError> for SergenError {
    fn from(err: ReqwestError) -> Self {
        SergenError::InstallationError(err.to_string())
    }
}