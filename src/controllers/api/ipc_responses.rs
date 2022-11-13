/// This module defines respones that are sent to the API.

use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize_repr)]
#[repr(u16)]
/// Represents a status code that is sent back to the API.
pub enum ApiStatus {
    Success = 0,
    InvalidEncoding = 1,
    InvalidRequestHead = 2,
    InvalidRequestBody = 3,
    InvalidRequestArgs = 4
}

#[derive(Serialize)]
/// Represents a response that is sent to the API. This structure wraps header
/// values and the body.
pub struct ApiResponse {
    /// The response error code to the given request.
    pub status: ApiStatus,

    /// The body of the error code.
    pub body: String
}

#[derive(Serialize)]
/// Represents a body returned upon an error.
pub struct ApiIpcErrorResponseBody {
    pub message: String
}

#[derive(Serialize)]
/// Represents a body returned upon successful LED setting.
pub struct ApiIpcLedResponseBody {}

#[derive(Serialize)]
/// Represents a body returned upon successful velocity setting.
pub struct ApiIpcVelResponseBody {}
