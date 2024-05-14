#[derive(Debug)]
pub enum ApiError {
    WikiError { status: u16, message: String},
    SurfError(surf::Error),
    SerdeJsonError(serde_json::Error),
    SerdeParseError(String)
}

impl From<surf::Error> for ApiError {
    fn from(err: surf::Error) -> Self {
        ApiError::SurfError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::SerdeJsonError(err)
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::SerdeParseError(err)
    }
}