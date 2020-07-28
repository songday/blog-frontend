use thiserror::Error as ThisError;

pub(crate) mod request;

/// Define all possible errors
#[derive(ThisError, Clone, Debug)]
pub(crate) enum Error {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    // #[error("Unprocessable Entity: {0:?}")]
    // UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    InternalServerError,

    /// serde deserialize error
    #[error("Deserialize Error")]
    DeserializeError,

    /// request error
    #[error("Http Request Error")]
    RequestError,
}